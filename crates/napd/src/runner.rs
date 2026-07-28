use std::{
    collections::VecDeque,
    fs,
    path::Path,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use nmp_native_runtime_ffi::{
    ArtifactCoordinate, ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, RuntimeConfig,
    RuntimeController, RuntimeEvent, RuntimeExecutionProfile, RuntimeGrantDecision,
    RuntimeObservation, RuntimeObservationFrame, RuntimeObserver, RuntimeSensitivity,
    RuntimeSnapshotProjection, VerifiedRead,
};
use serde::Serialize;

const AUTHOR: &str = "266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5";
const D_TAG: &str = "good-morning";
const AGGREGATE_HASH: &str = "828a6df02afd56782ea20f805084acce65c53f7c37554948c1e0a64aa5a2b0a8";
const INDEX_DIGEST: &str = "ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808";
const SURFACE_TOKEN: &str = "uzel-surface-1-generation-1";
const ARTIFACT_BASE_URL: &str = "nmp-artifact://828a6df0-2afd-4678-a20f-805084acce65/";
const MAXIMUM_ENVELOPE_BYTES: usize = 64 * 1_024;
const MAXIMUM_VERIFIED_DOCUMENT_BYTES: u64 = 512 * 1_024;
const MAXIMUM_BUFFERED_EVENTS: usize = 256;
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
const EVENT: &[u8] = include_bytes!("../../../fixtures/good-morning/event.json");
const INDEX: &[u8] = include_bytes!("../../../fixtures/good-morning/index.html");
const AVAILABLE_DOMAINS: [&str; 4] = ["shell", "identity", "inc", "outbox"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLaunch {
    pub surface_token: String,
    pub artifact_base_url: String,
    pub artifact_html: String,
    pub title: String,
    pub author: String,
    pub d_tag: String,
    pub aggregate_hash: String,
    pub domains: Vec<String>,
    pub unavailable_domains: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum RunnerError {
    #[error("runtime directory could not be created: {0}")]
    RuntimeDirectory(std::io::Error),
    #[error("runtime could not open: {0}")]
    RuntimeOpen(String),
    #[error("fixture verification refused: {0}")]
    Verification(String),
    #[error("verified fixture identity does not match pinned coordinate")]
    IdentityMismatch,
    #[error("runtime did not create exactly one fixture session")]
    SessionMissing,
    #[error("verified fixture document was refused: {0}")]
    VerifiedRead(String),
    #[error("verified fixture document is not UTF-8: {0}")]
    DocumentEncoding(std::string::FromUtf8Error),
    #[error("surface token is not mapped by the trusted host")]
    UnknownSurface,
    #[error("envelope exceeds the {MAXIMUM_ENVELOPE_BYTES}-byte host limit")]
    EnvelopeTooLarge,
    #[error("runtime produced no response before the bounded deadline")]
    ResponseTimeout,
    #[error("runtime observation could not start: {0}")]
    Observation(String),
}

#[derive(Debug)]
struct ExactFixtureSource;

impl ArtifactSource for ExactFixtureSource {
    fn fetch(&self, request: ArtifactFetchRequest) -> ArtifactFetchResponse {
        let accepted = request.logical_path == "/index.html"
            && request.expected_sha256 == INDEX_DIGEST
            && request.maximum_bytes >= INDEX.len() as u64
            && !request.redirects_allowed;
        ArtifactFetchResponse::Body {
            source_url: request.candidate_urls.first().cloned().unwrap_or_default(),
            http_status: if accepted { 200 } else { 404 },
            bytes: if accepted { INDEX.to_vec() } else { Vec::new() },
        }
    }
}

#[derive(Debug, Default)]
struct EventBuffer {
    events: Mutex<VecDeque<RuntimeEvent>>,
    changed: Condvar,
}

impl EventBuffer {
    fn latest_sequence(&self) -> u64 {
        self.events
            .lock()
            .expect("event buffer poisoned")
            .back()
            .map_or(0, |event| event.sequence)
    }

    fn response_after(&self, cursor: u64, session_id: u64) -> Option<String> {
        let deadline = Instant::now() + RESPONSE_TIMEOUT;
        let mut events = self.events.lock().expect("event buffer poisoned");
        loop {
            if let Some(response) = events.iter().find_map(|event| {
                (event.sequence > cursor && event.session_id == Some(session_id))
                    .then(|| event.response_json.clone())
                    .flatten()
            }) {
                return Some(response);
            }
            let now = Instant::now();
            if now >= deadline {
                return None;
            }
            let (next, timeout) = self
                .changed
                .wait_timeout(events, deadline.saturating_duration_since(now))
                .expect("event buffer poisoned");
            events = next;
            if timeout.timed_out() {
                return None;
            }
        }
    }
}

#[derive(Debug)]
struct EventSink(Arc<EventBuffer>);

impl RuntimeObserver for EventSink {
    fn update(&self, frame: RuntimeObservationFrame) {
        if frame.events.is_empty() {
            return;
        }
        let mut events = self.0.events.lock().expect("event buffer poisoned");
        for event in frame.events {
            if events.len() == MAXIMUM_BUFFERED_EVENTS {
                events.pop_front();
            }
            events.push_back(event);
        }
        self.0.changed.notify_all();
    }
}

pub struct LinuxRunner {
    controller: Arc<RuntimeController>,
    _observation: Arc<RuntimeObservation>,
    events: Arc<EventBuffer>,
    surface: Option<(u64, SurfaceLaunch)>,
}

impl std::fmt::Debug for LinuxRunner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LinuxRunner")
            .field("surface", &self.surface.as_ref().map(|(_, launch)| launch))
            .finish_non_exhaustive()
    }
}

impl LinuxRunner {
    pub fn open(runtime_root: impl AsRef<Path>) -> Result<Self, RunnerError> {
        let runtime_root = runtime_root.as_ref();
        fs::create_dir_all(runtime_root).map_err(RunnerError::RuntimeDirectory)?;
        let controller = RuntimeController::open(
            RuntimeConfig {
                runtime_store_path: runtime_root.join("runtime.sqlite3").display().to_string(),
                nmp_store_path: None,
                artifact_cache_path: runtime_root.join("artifacts").display().to_string(),
                ..RuntimeConfig::default()
            },
            Box::new(ExactFixtureSource),
        )
        .map_err(|error| RunnerError::RuntimeOpen(error.to_string()))?;
        let events = Arc::new(EventBuffer::default());
        let observation = Arc::clone(&controller).observe(Box::new(EventSink(Arc::clone(&events))));
        let observation = observation.observation.ok_or_else(|| {
            RunnerError::Observation(
                observation
                    .refusal
                    .map_or_else(|| "unknown refusal".to_owned(), |refusal| refusal.detail),
            )
        })?;
        Ok(Self {
            controller,
            _observation: observation,
            events,
            surface: None,
        })
    }

    pub fn start_fixture(&mut self) -> Result<SurfaceLaunch, RunnerError> {
        if let Some((_, launch)) = &self.surface {
            return Ok(launch.clone());
        }
        let verification = self.controller.verify_artifact(
            EVENT.to_vec(),
            ArtifactCoordinate::Named {
                author: AUTHOR.to_owned(),
                d_tag: D_TAG.to_owned(),
            },
        );
        let artifact = verification.artifact.ok_or_else(|| {
            RunnerError::Verification(
                verification
                    .refusal
                    .map_or_else(|| "unknown refusal".to_owned(), |refusal| refusal.detail),
            )
        })?;
        if artifact.author() != AUTHOR
            || artifact.d_tag().as_deref() != Some(D_TAG)
            || artifact.aggregate_hash() != AGGREGATE_HASH
        {
            return Err(RunnerError::IdentityMismatch);
        }
        self.controller.install(Arc::clone(&artifact));
        for domain in AVAILABLE_DOMAINS {
            self.controller.set_grant(
                Arc::clone(&artifact),
                domain.to_owned(),
                RuntimeSensitivity::Ordinary,
                RuntimeGrantDecision::AllowExactBuild,
            );
        }
        self.controller
            .launch(artifact, RuntimeExecutionProfile::Legacy);
        let snapshot = match self.controller.snapshot() {
            RuntimeSnapshotProjection::Snapshot { snapshot } => snapshot,
            RuntimeSnapshotProjection::Refused { refusal, .. } => {
                return Err(RunnerError::Verification(refusal.detail));
            }
        };
        let session = snapshot
            .sessions
            .into_iter()
            .find(|session| {
                session.author == AUTHOR
                    && session.d_tag == D_TAG
                    && session.aggregate_hash == AGGREGATE_HASH
            })
            .ok_or(RunnerError::SessionMissing)?;
        let artifact_html = match self.controller.read_verified(
            session.id,
            "/index.html".to_owned(),
            MAXIMUM_VERIFIED_DOCUMENT_BYTES,
        ) {
            VerifiedRead::Bytes { bytes, .. } => {
                String::from_utf8(bytes).map_err(RunnerError::DocumentEncoding)?
            }
            VerifiedRead::Refused { refusal } => {
                return Err(RunnerError::VerifiedRead(refusal.detail));
            }
        };
        let launch = SurfaceLaunch {
            surface_token: SURFACE_TOKEN.to_owned(),
            artifact_base_url: ARTIFACT_BASE_URL.to_owned(),
            artifact_html,
            title: "Good Morning Protocol".to_owned(),
            author: AUTHOR.to_owned(),
            d_tag: D_TAG.to_owned(),
            aggregate_hash: AGGREGATE_HASH.to_owned(),
            domains: session.domains,
            unavailable_domains: session.unavailable_domains,
        };
        self.surface = Some((session.id, launch.clone()));
        Ok(launch)
    }

    pub fn forward_from_surface(
        &self,
        surface_token: &str,
        envelope: &[u8],
    ) -> Result<String, RunnerError> {
        if envelope.len() > MAXIMUM_ENVELOPE_BYTES {
            return Err(RunnerError::EnvelopeTooLarge);
        }
        let (session_id, _) = self
            .surface
            .as_ref()
            .filter(|(_, launch)| launch.surface_token == surface_token)
            .ok_or(RunnerError::UnknownSurface)?;
        let cursor = self.events.latest_sequence();
        self.controller
            .mapped_envelope(*session_id, envelope.to_vec());
        self.events
            .response_after(cursor, *session_id)
            .ok_or(RunnerError::ResponseTimeout)
    }
}

impl Drop for LinuxRunner {
    fn drop(&mut self) {
        self.controller.close();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tempfile::TempDir;

    use super::*;

    const SLICE_THREE_AUTHOR: &str =
        "2c5cfba1117344eecebb730d440380ede068512d0c3982807e22075ef59ee362";
    const FOLLOW_LIST_EVENT: &[u8] = include_bytes!("../../../fixtures/follow-list/event.json");
    const FOLLOW_LIST_INDEX: &[u8] = include_bytes!("../../../fixtures/follow-list/index.html");
    const PROFILE_CARD_EVENT: &[u8] = include_bytes!("../../../fixtures/profile-card/event.json");
    const PROFILE_CARD_INDEX: &[u8] = include_bytes!("../../../fixtures/profile-card/index.html");
    const HOSTILE_EGRESS_EVENT: &[u8] =
        include_bytes!("../../../fixtures/hostile-egress/event.json");
    const HOSTILE_EGRESS_INDEX: &[u8] =
        include_bytes!("../../../fixtures/hostile-egress/index.html");

    #[derive(Debug)]
    struct SliceThreeFixtureSource;

    impl ArtifactSource for SliceThreeFixtureSource {
        fn fetch(&self, request: ArtifactFetchRequest) -> ArtifactFetchResponse {
            let bytes = match request.expected_sha256.as_str() {
                "3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4" => {
                    Some(FOLLOW_LIST_INDEX)
                }
                "c5c33e1e7dc755c5dafdfbd5357c10e3d409f7f3addf2138edc5de1f9b5dd284" => {
                    Some(PROFILE_CARD_INDEX)
                }
                "2687d474ea260f00c56c6861558cea4d10b972fa3aa39bfe1268d0375a539e06" => {
                    Some(HOSTILE_EGRESS_INDEX)
                }
                _ => None,
            };
            let accepted = request.logical_path == "/index.html"
                && bytes.is_some_and(|body| request.maximum_bytes >= body.len() as u64)
                && !request.redirects_allowed;
            ArtifactFetchResponse::Body {
                source_url: request.candidate_urls.first().cloned().unwrap_or_default(),
                http_status: if accepted { 200 } else { 404 },
                bytes: if accepted {
                    bytes.unwrap_or_default().to_vec()
                } else {
                    Vec::new()
                },
            }
        }
    }

    fn launch_slice_three_fixture(
        controller: &Arc<RuntimeController>,
        event: &[u8],
        d_tag: &str,
        domains: &[&str],
    ) -> u64 {
        let artifact = controller
            .verify_artifact(
                event.to_vec(),
                ArtifactCoordinate::Named {
                    author: SLICE_THREE_AUTHOR.to_owned(),
                    d_tag: d_tag.to_owned(),
                },
            )
            .artifact
            .unwrap_or_else(|| panic!("signed fixture {d_tag} verifies"));
        controller.install(Arc::clone(&artifact));
        for domain in std::iter::once("shell").chain(domains.iter().copied()) {
            controller.set_grant(
                Arc::clone(&artifact),
                domain.to_owned(),
                RuntimeSensitivity::Ordinary,
                RuntimeGrantDecision::AllowExactBuild,
            );
        }
        controller.launch(artifact, RuntimeExecutionProfile::Legacy);
        match controller.snapshot() {
            RuntimeSnapshotProjection::Snapshot { snapshot } => snapshot
                .sessions
                .into_iter()
                .find(|session| session.d_tag == d_tag)
                .map(|session| session.id)
                .unwrap_or_else(|| panic!("fixture session {d_tag} exists")),
            RuntimeSnapshotProjection::Refused { refusal, .. } => {
                panic!("snapshot refused: {}", refusal.detail)
            }
        }
    }

    #[test]
    fn verified_fixture_handshakes_through_runtime() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let launch = runner.start_fixture().unwrap();
        assert_eq!(launch.aggregate_hash, AGGREGATE_HASH);
        assert_eq!(launch.domains, ["identity", "inc", "outbox", "shell"]);
        assert_eq!(launch.unavailable_domains, ["link", "resource", "theme"]);

        let response = runner
            .forward_from_surface(&launch.surface_token, br#"{"type":"shell.ready"}"#)
            .unwrap();
        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["type"], "shell.init");
        assert_eq!(
            response["capabilities"]["domains"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
    }

    #[test]
    fn payload_identity_cannot_select_surface_or_session() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let launch = runner.start_fixture().unwrap();
        let forged = br#"{"type":"shell.ready","session":"1","principal":"forged"}"#;

        assert!(matches!(
            runner.forward_from_surface("attacker-chosen-surface", br#"{"type":"shell.ready"}"#),
            Err(RunnerError::UnknownSurface)
        ));
        assert!(matches!(
            runner.forward_from_surface(&launch.surface_token, forged),
            Err(RunnerError::ResponseTimeout)
        ));
    }

    #[test]
    fn slice_three_signed_single_file_artifacts_verify_and_read_exactly() {
        let fixtures = [
            (
                "follow-list",
                "eaf4e565642e5cd055c8f69bea832d39701d04d3a820f5a5753f39bb3651ea9a",
                FOLLOW_LIST_EVENT,
                FOLLOW_LIST_INDEX,
                &["identity", "inc"][..],
            ),
            (
                "profile-card",
                "1c17d749e288d38db10e570c9bbb66154e528513e323c8437ba2a58a67e9a413",
                PROFILE_CARD_EVENT,
                PROFILE_CARD_INDEX,
                &["inc", "outbox"][..],
            ),
            (
                "egress-probe",
                "0514a2812f5f1f1b4c9d20cd4ffca7f016f434e11d108eef6b5293a4c61e7670",
                HOSTILE_EGRESS_EVENT,
                HOSTILE_EGRESS_INDEX,
                &[][..],
            ),
        ];

        for (d_tag, aggregate_hash, event, index, domains) in fixtures {
            let temp = TempDir::new().unwrap();
            let controller = RuntimeController::open(
                RuntimeConfig {
                    runtime_store_path: temp.path().join("runtime.sqlite3").display().to_string(),
                    nmp_store_path: None,
                    artifact_cache_path: temp.path().join("artifacts").display().to_string(),
                    ..RuntimeConfig::default()
                },
                Box::new(SliceThreeFixtureSource),
            )
            .unwrap();
            let verification = controller.verify_artifact(
                event.to_vec(),
                ArtifactCoordinate::Named {
                    author: SLICE_THREE_AUTHOR.to_owned(),
                    d_tag: d_tag.to_owned(),
                },
            );
            let artifact = verification.artifact.unwrap_or_else(|| {
                panic!(
                    "signed fixture {d_tag} verifies: {:?}",
                    verification.refusal
                )
            });
            assert!(verification.refusal.is_none());
            assert_eq!(artifact.author(), SLICE_THREE_AUTHOR);
            assert_eq!(artifact.d_tag().as_deref(), Some(d_tag));
            assert_eq!(artifact.aggregate_hash(), aggregate_hash);

            controller.install(Arc::clone(&artifact));
            for domain in std::iter::once("shell").chain(domains.iter().copied()) {
                controller.set_grant(
                    Arc::clone(&artifact),
                    domain.to_owned(),
                    RuntimeSensitivity::Ordinary,
                    RuntimeGrantDecision::AllowExactBuild,
                );
            }
            controller.launch(artifact, RuntimeExecutionProfile::Legacy);
            let session = match controller.snapshot() {
                RuntimeSnapshotProjection::Snapshot { snapshot } => snapshot
                    .sessions
                    .into_iter()
                    .find(|session| session.d_tag == d_tag)
                    .expect("fixture session exists"),
                RuntimeSnapshotProjection::Refused { refusal, .. } => {
                    panic!("snapshot refused: {}", refusal.detail)
                }
            };
            for domain in domains {
                assert!(session.domains.iter().any(|granted| granted == domain));
            }
            match controller.read_verified(
                session.id,
                "/index.html".to_owned(),
                MAXIMUM_VERIFIED_DOCUMENT_BYTES,
            ) {
                VerifiedRead::Bytes { bytes, .. } => assert_eq!(bytes, index),
                VerifiedRead::Refused { refusal } => panic!("verified read refused: {refusal:?}"),
            }
            controller.close();
        }
    }

    #[test]
    fn profile_open_crosses_inc_with_runtime_owned_sender() {
        let temp = TempDir::new().unwrap();
        let controller = RuntimeController::open(
            RuntimeConfig {
                runtime_store_path: temp.path().join("runtime.sqlite3").display().to_string(),
                nmp_store_path: None,
                artifact_cache_path: temp.path().join("artifacts").display().to_string(),
                ..RuntimeConfig::default()
            },
            Box::new(SliceThreeFixtureSource),
        )
        .unwrap();
        let events = Arc::new(EventBuffer::default());
        let observation = Arc::clone(&controller).observe(Box::new(EventSink(Arc::clone(&events))));
        let _observation = observation.observation.expect("runtime observation starts");
        let follow_session = launch_slice_three_fixture(
            &controller,
            FOLLOW_LIST_EVENT,
            "follow-list",
            &["identity", "inc"],
        );
        let profile_session = launch_slice_three_fixture(
            &controller,
            PROFILE_CARD_EVENT,
            "profile-card",
            &["inc", "outbox"],
        );
        for session in [follow_session, profile_session] {
            let cursor = events.latest_sequence();
            controller.mapped_envelope(session, br#"{"type":"shell.ready"}"#.to_vec());
            let response = events
                .response_after(cursor, session)
                .expect("NAP-SHELL initialization response");
            let response: Value = serde_json::from_str(&response).unwrap();
            assert_eq!(response["type"], "shell.init");
        }

        let cursor = events.latest_sequence();
        controller.mapped_envelope(
            profile_session,
            br#"{"type":"inc.subscribe","id":"profile-open-sub","topic":"napplet:profile/open"}"#
                .to_vec(),
        );
        let response = events
            .response_after(cursor, profile_session)
            .expect("INC subscription response");
        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["type"], "inc.subscribe.result");

        let cursor = events.latest_sequence();
        controller.mapped_envelope(
            follow_session,
            br#"{"type":"inc.emit","topic":"napplet:profile/open","payload":{"version":1,"pubkey":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#
                .to_vec(),
        );
        let delivery = events
            .response_after(cursor, profile_session)
            .expect("INC profile-open delivery");
        let delivery: Value = serde_json::from_str(&delivery).unwrap();
        assert_eq!(delivery["type"], "inc.event");
        assert_eq!(delivery["topic"], "napplet:profile/open");
        assert_eq!(delivery["sender"], "follow-list");
        assert_eq!(delivery["payload"]["version"], 1);
        assert_eq!(delivery["payload"]["pubkey"], "a".repeat(64));
        controller.close();
    }
}
