use std::{
    collections::VecDeque,
    fs,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use napd_protocol::{Diagnostics, SurfaceMetadata};
use nmp_native_runtime_ffi::{
    ArtifactCoordinate, ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource,
    RuntimeAccountHandle, RuntimeConfig, RuntimeController, RuntimeEvent, RuntimeExecutionProfile,
    RuntimeGrantDecision, RuntimeObservation, RuntimeObservationFrame, RuntimeObserver,
    RuntimeRelayDiagnosticsObservation, RuntimeRelayDiagnosticsObserver,
    RuntimeRelayDiagnosticsSnapshot, RuntimeSensitivity, RuntimeSnapshotProjection, VerifiedRead,
};
use serde::{Deserialize, Serialize};

const AUTHOR: &str = "266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5";
const D_TAG: &str = "good-morning";
const AGGREGATE_HASH: &str = "828a6df02afd56782ea20f805084acce65c53f7c37554948c1e0a64aa5a2b0a8";
const INDEX_DIGEST: &str = "ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808";
const ARTIFACT_BASE_URL: &str = "nmp-artifact://828a6df0-2afd-4678-a20f-805084acce65/";
const MAXIMUM_ENVELOPE_BYTES: usize = 64 * 1_024;
const MAXIMUM_VERIFIED_DOCUMENT_BYTES: u64 = 512 * 1_024;
const MAXIMUM_BUFFERED_EVENTS: usize = 256;
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
const EVENT: &[u8] = include_bytes!("../../../fixtures/good-morning/event.json");
const INDEX: &[u8] = include_bytes!("../../../fixtures/good-morning/index.html");
const AVAILABLE_DOMAINS: [&str; 4] = ["shell", "identity", "inc", "outbox"];
const PRODUCT_STATE_VERSION: u8 = 0;
const MAXIMUM_PRODUCT_STATE_BYTES: u64 = 4_096;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProductState {
    version: u8,
    mode: String,
    active_read_identity: Option<String>,
    #[serde(default)]
    next_surface_generation: u64,
}

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

impl SurfaceLaunch {
    pub fn metadata(&self) -> SurfaceMetadata {
        SurfaceMetadata {
            surface_token: self.surface_token.clone(),
            artifact_base_url: self.artifact_base_url.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            d_tag: self.d_tag.clone(),
            aggregate_hash: self.aggregate_hash.clone(),
            domains: self.domains.clone(),
            unavailable_domains: self.unavailable_domains.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeMode {
    Fixture,
    Live,
}

impl RuntimeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixture => "fixture",
            Self::Live => "live",
        }
    }
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
    #[error("read identity was refused: {0}")]
    Identity(String),
    #[error("product state could not be loaded: {0}")]
    StateLoad(String),
    #[error("product state could not be persisted: {0}")]
    StatePersist(String),
    #[error("surface generation is exhausted")]
    SurfaceGenerationExhausted,
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

    #[cfg(test)]
    fn response_after(&self, cursor: u64, session_id: u64) -> Option<String> {
        self.response_matching_after(cursor, session_id, |_| true)
    }

    fn response_matching_after(
        &self,
        cursor: u64,
        session_id: u64,
        matches: impl Fn(&str) -> bool,
    ) -> Option<String> {
        let deadline = Instant::now() + RESPONSE_TIMEOUT;
        let mut events = self.events.lock().expect("event buffer poisoned");
        loop {
            if let Some(response) = events.iter().find_map(|event| {
                (event.sequence > cursor && event.session_id == Some(session_id))
                    .then(|| {
                        event
                            .response_json
                            .as_deref()
                            .filter(|response| matches(response))
                    })
                    .flatten()
                    .map(str::to_owned)
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

#[derive(Debug)]
struct RelayDiagnosticsSink;

impl RuntimeRelayDiagnosticsObserver for RelayDiagnosticsSink {
    fn update(&self, _snapshot: RuntimeRelayDiagnosticsSnapshot) {}
}

pub struct LinuxRunner {
    controller: Arc<RuntimeController>,
    observation: Arc<RuntimeObservation>,
    relay_observation: Arc<RuntimeRelayDiagnosticsObservation>,
    events: Arc<EventBuffer>,
    surface: Option<(u64, SurfaceLaunch)>,
    mode: RuntimeMode,
    state_path: std::path::PathBuf,
    next_surface_generation: u64,
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
        Self::open_configured(
            runtime_root,
            RuntimeMode::Fixture,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn open_live(
        runtime_root: impl AsRef<Path>,
        indexer_relays: Vec<String>,
        app_relays: Vec<String>,
        fallback_relays: Vec<String>,
        allowed_local_relay_hosts: Vec<String>,
    ) -> Result<Self, RunnerError> {
        Self::open_configured(
            runtime_root,
            RuntimeMode::Live,
            indexer_relays,
            app_relays,
            fallback_relays,
            allowed_local_relay_hosts,
        )
    }

    fn open_configured(
        runtime_root: impl AsRef<Path>,
        mode: RuntimeMode,
        indexer_relays: Vec<String>,
        app_relays: Vec<String>,
        fallback_relays: Vec<String>,
        allowed_local_relay_hosts: Vec<String>,
    ) -> Result<Self, RunnerError> {
        let runtime_root = runtime_root.as_ref();
        fs::create_dir_all(runtime_root).map_err(RunnerError::RuntimeDirectory)?;
        let controller = RuntimeController::open(
            RuntimeConfig {
                runtime_store_path: runtime_root.join("runtime.sqlite3").display().to_string(),
                nmp_store_path: Some(runtime_root.join("nmp.redb").display().to_string()),
                artifact_cache_path: runtime_root.join("artifacts").display().to_string(),
                indexer_relays,
                app_relays,
                fallback_relays,
                allowed_local_relay_hosts,
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
        let relay_observation =
            controller.observe_relay_diagnostics(Box::new(RelayDiagnosticsSink));
        let relay_observation = relay_observation.observation.ok_or_else(|| {
            RunnerError::Observation(relay_observation.refusal.map_or_else(
                || "unknown relay diagnostics refusal".to_owned(),
                |refusal| refusal.detail,
            ))
        })?;
        let mut runner = Self {
            controller,
            observation,
            relay_observation,
            events,
            surface: None,
            mode,
            state_path: runtime_root.join("uzel-state.json"),
            next_surface_generation: 0,
        };
        runner.restore_product_state()?;
        runner.persist_product_state()?;
        Ok(runner)
    }

    pub fn mode(&self) -> RuntimeMode {
        self.mode
    }

    pub fn active_surface(&self) -> Option<&str> {
        self.surface
            .as_ref()
            .map(|(_, launch)| launch.surface_token.as_str())
    }

    pub fn get_read_identity(&self) -> Result<Option<String>, RunnerError> {
        let update = self.controller.account_snapshot();
        if !update.accepted {
            return Err(RunnerError::Identity(format!("{:?}", update.failure)));
        }
        Ok(update
            .snapshot
            .and_then(|snapshot| snapshot.active_public_key))
    }

    pub fn set_read_identity(&mut self, public_identity: String) -> Result<String, RunnerError> {
        let before = self.controller.account_snapshot();
        if !before.accepted {
            return Err(RunnerError::Identity(format!("{:?}", before.failure)));
        }
        let before = before
            .snapshot
            .ok_or_else(|| RunnerError::Identity("NMP returned no account snapshot".to_owned()))?;
        let previous_identity = before.active_public_key;
        let previous_handle = previous_identity.as_ref().and_then(|public_key| {
            before
                .local_accounts
                .into_iter()
                .find(|handle| handle.public_key == *public_key)
        });
        if previous_identity.is_some() && previous_handle.is_none() {
            return Err(RunnerError::Identity(
                "NMP active identity has no owned installation".to_owned(),
            ));
        }

        let registered = self.controller.register_read_only_account(public_identity);
        let handle = registered
            .handle
            .ok_or_else(|| RunnerError::Identity(format!("{:?}", registered.failure)))?;
        if previous_identity.as_deref() == Some(handle.public_key.as_str()) {
            self.persist_product_state()?;
            return Ok(handle.public_key);
        }

        let active = self.activate_account(handle.clone())?;
        if let Err(persist_error) = self.persist_product_state() {
            self.rollback_identity(previous_handle, handle)?;
            return Err(persist_error);
        }
        Ok(active)
    }

    fn activate_read_identity(&self, public_identity: String) -> Result<String, RunnerError> {
        let registered = self.controller.register_read_only_account(public_identity);
        let handle = registered
            .handle
            .ok_or_else(|| RunnerError::Identity(format!("{:?}", registered.failure)))?;
        self.activate_account(handle)
    }

    fn activate_account(&self, handle: RuntimeAccountHandle) -> Result<String, RunnerError> {
        let activated = self.controller.activate_local_account(handle);
        if !activated.accepted {
            return Err(RunnerError::Identity(format!("{:?}", activated.failure)));
        }
        activated
            .snapshot
            .and_then(|snapshot| snapshot.active_public_key)
            .ok_or_else(|| RunnerError::Identity("NMP returned no active public key".to_owned()))
    }

    fn rollback_identity(
        &self,
        previous: Option<RuntimeAccountHandle>,
        replacement: RuntimeAccountHandle,
    ) -> Result<(), RunnerError> {
        let restored = match previous {
            Some(handle) => self.controller.activate_local_account(handle),
            None => self.controller.logout_local_account(),
        };
        if !restored.accepted {
            return Err(RunnerError::Identity(format!(
                "identity persistence failed and rollback was refused: {:?}",
                restored.failure
            )));
        }
        let removed = self.controller.remove_local_account(replacement);
        if !removed.accepted {
            return Err(RunnerError::Identity(format!(
                "identity rollback restored the prior selection but replacement removal was refused: {:?}",
                removed.failure
            )));
        }
        Ok(())
    }

    fn restore_product_state(&mut self) -> Result<(), RunnerError> {
        let metadata = match fs::symlink_metadata(&self.state_path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(RunnerError::StateLoad(error.to_string())),
        };
        if !metadata.file_type().is_file() {
            return Err(RunnerError::StateLoad(
                "state path is not a regular file".to_owned(),
            ));
        }
        if metadata.len() > MAXIMUM_PRODUCT_STATE_BYTES {
            return Err(RunnerError::StateLoad(format!(
                "state exceeds {MAXIMUM_PRODUCT_STATE_BYTES} bytes"
            )));
        }
        let bytes = fs::read(&self.state_path)
            .map_err(|error| RunnerError::StateLoad(error.to_string()))?;
        let state: ProductState = serde_json::from_slice(&bytes)
            .map_err(|error| RunnerError::StateLoad(error.to_string()))?;
        if state.version != PRODUCT_STATE_VERSION {
            return Err(RunnerError::StateLoad(format!(
                "unsupported state version {}",
                state.version
            )));
        }
        if state.mode != RuntimeMode::Fixture.as_str() && state.mode != RuntimeMode::Live.as_str() {
            return Err(RunnerError::StateLoad(format!(
                "unsupported product mode {}",
                state.mode
            )));
        }
        if let Some(public_identity) = state.active_read_identity {
            self.activate_read_identity(public_identity)?;
        }
        self.next_surface_generation = state.next_surface_generation;
        Ok(())
    }

    fn persist_product_state(&self) -> Result<(), RunnerError> {
        let state = ProductState {
            version: PRODUCT_STATE_VERSION,
            mode: self.mode.as_str().to_owned(),
            active_read_identity: self.get_read_identity()?,
            next_surface_generation: self.next_surface_generation,
        };
        let bytes = serde_json::to_vec(&state)
            .map_err(|error| RunnerError::StatePersist(error.to_string()))?;
        if bytes.len() as u64 > MAXIMUM_PRODUCT_STATE_BYTES {
            return Err(RunnerError::StatePersist("state is oversized".to_owned()));
        }
        let temporary = self
            .state_path
            .with_extension(format!("json.{}.tmp", std::process::id()));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)
            .map_err(|error| RunnerError::StatePersist(error.to_string()))?;
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| RunnerError::StatePersist(error.to_string()))?;
        fs::rename(&temporary, &self.state_path)
            .map_err(|error| RunnerError::StatePersist(error.to_string()))?;
        Ok(())
    }

    pub fn diagnostics(&self) -> Result<Diagnostics, RunnerError> {
        let snapshot = match self.controller.snapshot() {
            RuntimeSnapshotProjection::Snapshot { snapshot } => snapshot,
            RuntimeSnapshotProjection::Refused { refusal, .. } => {
                return Err(RunnerError::Verification(refusal.detail));
            }
        };
        let relay = self.controller.relay_diagnostics();
        Ok(Diagnostics {
            snapshot_revision: snapshot.revision,
            active_sessions: snapshot.sessions.len() as u64,
            active_identity: self.get_read_identity()?,
            relay_revision: relay.revision,
            observing_relays: relay.observing,
            relays: relay.relays.len() as u64,
            omitted_relays: relay.omitted_relays,
            store_degraded: bounded_diagnostic(relay.store_degraded),
            transport_degraded: bounded_diagnostic(relay.transport_degraded),
        })
    }

    pub fn start_fixture(&mut self) -> Result<SurfaceLaunch, RunnerError> {
        if let Some((_, launch)) = &self.surface {
            return Ok(launch.clone());
        }
        let previous_generation = self.next_surface_generation;
        self.next_surface_generation = previous_generation
            .checked_add(1)
            .ok_or(RunnerError::SurfaceGenerationExhausted)?;
        if let Err(error) = self.persist_product_state() {
            self.next_surface_generation = previous_generation;
            return Err(error);
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
                    && session.state.starts_with("running")
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
            surface_token: format!("uzel-surface-1-generation-{}", self.next_surface_generation),
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
        let expectation = ResponseExpectation::from_envelope(envelope);
        self.controller
            .mapped_envelope(*session_id, envelope.to_vec());
        self.events
            .response_matching_after(cursor, *session_id, |response| {
                expectation.matches(response)
            })
            .ok_or(RunnerError::ResponseTimeout)
    }

    pub fn stop_fixture(&mut self, surface_token: &str) -> Result<(), RunnerError> {
        let (session_id, _) = self
            .surface
            .as_ref()
            .filter(|(_, launch)| launch.surface_token == surface_token)
            .ok_or(RunnerError::UnknownSurface)?;
        self.controller.stop(*session_id);
        self.surface = None;
        Ok(())
    }
}

fn bounded_diagnostic(detail: Option<String>) -> Option<String> {
    const MAXIMUM_DIAGNOSTIC_BYTES: usize = 512;
    detail.map(|mut value| {
        if value.len() > MAXIMUM_DIAGNOSTIC_BYTES {
            let mut boundary = MAXIMUM_DIAGNOSTIC_BYTES;
            while !value.is_char_boundary(boundary) {
                boundary -= 1;
            }
            value.truncate(boundary);
        }
        value
    })
}

#[derive(Debug)]
enum ResponseExpectation {
    Id(String),
    Type(&'static str),
    Any,
}

impl ResponseExpectation {
    fn from_envelope(envelope: &[u8]) -> Self {
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(envelope) else {
            return Self::Any;
        };
        if let Some(id) = value.get("id").and_then(serde_json::Value::as_str) {
            return Self::Id(id.to_owned());
        }
        if value.get("type").and_then(serde_json::Value::as_str) == Some("shell.ready") {
            return Self::Type("shell.init");
        }
        Self::Any
    }

    fn matches(&self, response: &str) -> bool {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(response) else {
            return false;
        };
        match self {
            Self::Id(expected) => {
                value.get("id").and_then(serde_json::Value::as_str) == Some(expected.as_str())
            }
            Self::Type(expected) => {
                value.get("type").and_then(serde_json::Value::as_str) == Some(*expected)
            }
            Self::Any => true,
        }
    }
}

impl Drop for LinuxRunner {
    fn drop(&mut self) {
        self.observation.stop();
        self.relay_observation.stop();
        self.controller.close();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{TcpListener, TcpStream},
        os::unix::fs::symlink,
        process::{Child, Command, Stdio},
        thread,
    };

    use nmp_native_runtime_ffi::{
        NativeConfigCommit, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest,
    };
    use serde_json::Value;
    use tempfile::TempDir;

    use super::*;

    const SLICE_THREE_AUTHOR: &str =
        "5ffaf74a636594d5995750526f67a0db34b1c49db9433844ecfb981af7ba69b2";
    const FOLLOW_LIST_EVENT: &[u8] = include_bytes!("../../../fixtures/follow-list/event.json");
    const FOLLOW_LIST_INDEX: &[u8] = include_bytes!("../../../fixtures/follow-list/index.html");
    const PROFILE_CARD_EVENT: &[u8] = include_bytes!("../../../fixtures/profile-card/event.json");
    const PROFILE_CARD_INDEX: &[u8] = include_bytes!("../../../fixtures/profile-card/index.html");
    const HOSTILE_EGRESS_EVENT: &[u8] =
        include_bytes!("../../../fixtures/hostile-egress/event.json");
    const HOSTILE_EGRESS_INDEX: &[u8] =
        include_bytes!("../../../fixtures/hostile-egress/index.html");
    const LIVE_IDENTITY: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";

    #[derive(Debug)]
    struct SliceThreeFixtureSource;

    #[derive(Debug)]
    struct AcceptSettings;

    struct RelayProcess(Child);

    impl Drop for RelayProcess {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    impl NativeSettingsExecutor for AcceptSettings {
        fn try_open(&self, _request: NativeSettingsRequest) -> NativeSettingsOpenResult {
            NativeSettingsOpenResult::Accepted
        }
    }

    impl ArtifactSource for SliceThreeFixtureSource {
        fn fetch(&self, request: ArtifactFetchRequest) -> ArtifactFetchResponse {
            let bytes = match request.expected_sha256.as_str() {
                "3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4" => {
                    Some(FOLLOW_LIST_INDEX)
                }
                "eeb037774dcc43faf6e0e13a9cf67aae8684b34c9c52921bcbd511739c46fa63" => {
                    Some(PROFILE_CARD_INDEX)
                }
                "94fd9d4e5ab363b17be0a6baba4b19783fabe115bced157fc081087039f1a4a9" => {
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

    fn start_fixture_relay() -> (RelayProcess, String) {
        let reservation = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = reservation.local_addr().unwrap().port();
        drop(reservation);
        let events =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/nostr/live-events.jsonl");
        let port_string = port.to_string();
        let events_string = events.display().to_string();
        let child = Command::new("nak")
            .args([
                "serve",
                "--hostname",
                "127.0.0.1",
                "--port",
                &port_string,
                "--events",
                &events_string,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("nak is installed for the explicit live NMP probe");
        let deadline = Instant::now() + Duration::from_secs(3);
        while TcpStream::connect(("127.0.0.1", port)).is_err() {
            assert!(Instant::now() < deadline, "nak fixture relay did not bind");
            thread::sleep(Duration::from_millis(20));
        }
        (RelayProcess(child), format!("ws://127.0.0.1:{port}"))
    }

    fn launch_identity_surface(runner: &mut LinuxRunner) -> SurfaceLaunch {
        let launch = runner.start_fixture().unwrap();
        let response = runner
            .forward_from_surface(&launch.surface_token, br#"{"type":"shell.ready"}"#)
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&response).unwrap()["type"],
            "shell.init"
        );
        launch
    }

    fn identity_query(
        runner: &LinuxRunner,
        launch: &SurfaceLaunch,
        action: &str,
        id: &str,
    ) -> Value {
        let request = serde_json::json!({"type": format!("identity.{action}"), "id": id});
        let response = runner
            .forward_from_surface(&launch.surface_token, request.to_string().as_bytes())
            .unwrap();
        serde_json::from_str(&response).unwrap()
    }

    fn eventually_identity_query(
        runner: &LinuxRunner,
        launch: &SurfaceLaunch,
        action: &str,
        accepts: impl Fn(&Value) -> bool,
    ) -> Value {
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut attempt = 0_u64;
        loop {
            let response =
                identity_query(runner, launch, action, &format!("live-{action}-{attempt}"));
            if accepts(&response) {
                return response;
            }
            assert!(
                Instant::now() < deadline,
                "NMP live refresh did not satisfy identity.{action}: {response}"
            );
            attempt += 1;
            thread::sleep(Duration::from_millis(50));
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
    fn nmp_parses_and_persists_the_active_read_identity() {
        let temp = TempDir::new().unwrap();
        {
            let mut runner = LinuxRunner::open(temp.path()).unwrap();
            assert_eq!(runner.get_read_identity().unwrap(), None);
            assert_eq!(runner.set_read_identity(AUTHOR.to_owned()).unwrap(), AUTHOR);
            assert!(matches!(
                runner.set_read_identity(AUTHOR.to_ascii_uppercase()),
                Err(RunnerError::Identity(_))
            ));
        }
        let runner = LinuxRunner::open(temp.path()).unwrap();
        assert_eq!(runner.get_read_identity().unwrap().as_deref(), Some(AUTHOR));
    }

    #[test]
    fn restarted_daemon_gets_a_new_surface_generation() {
        let temp = TempDir::new().unwrap();
        let first = {
            let mut runner = LinuxRunner::open(temp.path()).unwrap();
            let first = runner.start_fixture().unwrap();
            runner.stop_fixture(&first.surface_token).unwrap();
            first
        };
        let mut restarted = LinuxRunner::open(temp.path()).unwrap();
        let second = restarted.start_fixture().unwrap();
        assert_ne!(first.surface_token, second.surface_token);
    }

    #[test]
    fn identity_activation_rolls_back_when_product_state_cannot_persist() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        assert_eq!(runner.set_read_identity(AUTHOR.to_owned()).unwrap(), AUTHOR);
        let temporary = temp
            .path()
            .join(format!("uzel-state.json.{}.tmp", std::process::id()));
        fs::write(&temporary, b"occupied").unwrap();

        assert!(matches!(
            runner.set_read_identity(LIVE_IDENTITY.to_owned()),
            Err(RunnerError::StatePersist(_))
        ));
        assert_eq!(runner.get_read_identity().unwrap().as_deref(), Some(AUTHOR));
    }

    #[test]
    fn product_state_symlink_is_refused() {
        let temp = TempDir::new().unwrap();
        let external = temp.path().join("external-state.json");
        fs::write(
            &external,
            br#"{"version":0,"mode":"fixture","active_read_identity":null}"#,
        )
        .unwrap();
        symlink(&external, temp.path().join("uzel-state.json")).unwrap();
        assert!(matches!(
            LinuxRunner::open(temp.path()),
            Err(RunnerError::StateLoad(_))
        ));
    }

    #[test]
    #[ignore = "requires the installed nak executable and loopback sockets"]
    fn live_nmp_refreshes_then_restarts_cache_first_without_a_second_cache() {
        let temp = TempDir::new().unwrap();
        let (relay, relay_url) = start_fixture_relay();
        {
            let mut runner = LinuxRunner::open_live(
                temp.path(),
                Vec::new(),
                Vec::new(),
                vec![relay_url.clone()],
                vec!["127.0.0.1".to_owned()],
            )
            .unwrap();
            assert_eq!(
                runner.set_read_identity(LIVE_IDENTITY.to_owned()).unwrap(),
                LIVE_IDENTITY
            );
            let launch = launch_identity_surface(&mut runner);
            let profile = eventually_identity_query(&runner, &launch, "getProfile", |response| {
                response["profile"]["name"] == "Alice"
            });
            assert_eq!(profile["type"], "identity.getProfile.result");
            assert_eq!(profile["profile"]["name"], "Alice");
            assert_eq!(profile["profile"]["displayName"], "Alice A.");
            assert_eq!(profile["profile"]["about"], "canonical");
            let follows = eventually_identity_query(&runner, &launch, "getFollows", |response| {
                response["pubkeys"]
                    .as_array()
                    .is_some_and(|items| items.len() == 2)
            });
            assert_eq!(follows["type"], "identity.getFollows.result");
            assert_eq!(
                follows["pubkeys"],
                serde_json::json!([
                    "04c915daefee38317fa734444acee390a8269fe5810b2241e5e6dd343dfbecc9",
                    "5d14b37435f05775bad136df0c51ccdcdc6f96482f0fea8404eeaf29ca5a8846"
                ])
            );
        }
        drop(relay);

        let mut runner = LinuxRunner::open_live(
            temp.path(),
            Vec::new(),
            Vec::new(),
            vec![relay_url],
            vec!["127.0.0.1".to_owned()],
        )
        .unwrap();
        assert_eq!(
            runner.get_read_identity().unwrap().as_deref(),
            Some(LIVE_IDENTITY)
        );
        let launch = launch_identity_surface(&mut runner);
        let cached = identity_query(&runner, &launch, "getProfile", "cached-profile");
        assert_eq!(cached["profile"]["name"], "Alice");
        assert_eq!(cached["profile"]["about"], "canonical");
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
                "9ee2d7bfebcd1c56f9c8c0e4641402e2d9ab7bed8c97c5d480cc77c04d5690cc",
                PROFILE_CARD_EVENT,
                PROFILE_CARD_INDEX,
                &["inc", "outbox"][..],
            ),
            (
                "egress-probe",
                "6dcafdf3a2622d7daa6544f1c668ffce7b5dd1f0c7984d658ae0803da93410c0",
                HOSTILE_EGRESS_EVENT,
                HOSTILE_EGRESS_INDEX,
                &["config"][..],
            ),
        ];

        for (d_tag, aggregate_hash, event, index, domains) in fixtures {
            let temp = TempDir::new().unwrap();
            let controller = RuntimeController::open_with_settings(
                RuntimeConfig {
                    runtime_store_path: temp.path().join("runtime.sqlite3").display().to_string(),
                    nmp_store_path: None,
                    artifact_cache_path: temp.path().join("artifacts").display().to_string(),
                    ..RuntimeConfig::default()
                },
                Box::new(SliceThreeFixtureSource),
                Box::new(AcceptSettings),
            )
            .unwrap();
            let events = Arc::new(EventBuffer::default());
            let observation =
                Arc::clone(&controller).observe(Box::new(EventSink(Arc::clone(&events))));
            let _observation = observation.observation.expect("runtime observation starts");
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
            if d_tag == "egress-probe" {
                let cursor = events.latest_sequence();
                controller.mapped_envelope(session.id, br#"{"type":"shell.ready"}"#.to_vec());
                let response = events
                    .response_after(cursor, session.id)
                    .expect("NAP-SHELL initialization response");
                let response: Value = serde_json::from_str(&response).unwrap();
                assert_eq!(response["type"], "shell.init");
                let sentinel = "http://127.0.0.1:43129/hostile-egress?run=fixture";
                let commit = controller.commit_config_values(NativeConfigCommit {
                    manifest_author: SLICE_THREE_AUTHOR.to_owned(),
                    d_tag: d_tag.to_owned(),
                    aggregate_hash: aggregate_hash.to_owned(),
                    session_id: session.id,
                    values_json: serde_json::json!({"sentinel": sentinel}).to_string(),
                });
                assert!(
                    commit.accepted,
                    "exact-session sentinel commit is accepted: {:?}",
                    commit.refusal
                );
                let cursor = events.latest_sequence();
                controller.mapped_envelope(
                    session.id,
                    br#"{"type":"config.get","id":"sentinel-get"}"#.to_vec(),
                );
                let response = events
                    .response_after(cursor, session.id)
                    .expect("source-bound config response");
                let response: Value = serde_json::from_str(&response).unwrap();
                assert_eq!(response["type"], "config.values");
                assert_eq!(response["values"]["sentinel"], sentinel);
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
