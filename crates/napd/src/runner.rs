use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use napd_protocol::{Diagnostics, RelayDiagnostic, RoutedEnvelope, SurfaceMetadata};
use nmp_native_runtime_ffi::{
    ArtifactCoordinate, NativeConfigCommit, NativeSettingsExecutor, NativeSettingsOpenResult,
    NativeSettingsRequest, RuntimeAccountHandle, RuntimeConfig, RuntimeController, RuntimeEvent,
    RuntimeExecutionProfile, RuntimeGrantDecision, RuntimeObservation, RuntimeObservationFrame,
    RuntimeObserver, RuntimeRelayAccess, RuntimeRelayDiagnosticsObservation,
    RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot, RuntimeRelayLane,
    RuntimeSensitivity, RuntimeSnapshotProjection, VerifiedRead,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::fixtures::{ExactFixtureSource, MAXIMUM_ACTIVE_FIXTURES, fixture_by_name};

const MAXIMUM_ENVELOPE_BYTES: usize = 64 * 1_024;
const MAXIMUM_VERIFIED_DOCUMENT_BYTES: u64 = 512 * 1_024;
const MAXIMUM_BUFFERED_EVENTS: usize = 256;
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
const PRODUCT_STATE_VERSION: u8 = 0;
const MAXIMUM_PRODUCT_STATE_BYTES: u64 = 4_096;

#[derive(Debug)]
enum StatePersistFailure {
    Unchanged(String),
    Replaced(String),
}

impl StatePersistFailure {
    fn into_runner(self) -> RunnerError {
        let detail = match self {
            Self::Unchanged(detail) | Self::Replaced(detail) => detail,
        };
        RunnerError::StatePersist(detail)
    }
}

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

fn relay_lane_name(lane: RuntimeRelayLane) -> &'static str {
    match lane {
        RuntimeRelayLane::Nip65Write => "nip65-write",
        RuntimeRelayLane::Nip65Read => "nip65-read",
        RuntimeRelayLane::Hint => "hint",
        RuntimeRelayLane::Provenance => "provenance",
        RuntimeRelayLane::UserConfigured => "user-configured",
        RuntimeRelayLane::IndexerDiscovery => "indexer-discovery",
        RuntimeRelayLane::GroupHost => "group-host",
        RuntimeRelayLane::DmInbox => "dm-inbox",
        RuntimeRelayLane::AppRelay => "app-relay",
        RuntimeRelayLane::Fallback => "fallback",
        RuntimeRelayLane::ExplicitPinned => "explicit-pinned",
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
    #[error("fixture name is not in the exact signed POC catalog")]
    UnknownFixture,
    #[error("active fixture capacity is {MAXIMUM_ACTIVE_FIXTURES}")]
    SurfaceCapacity,
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
    #[error(
        "hostile sentinel URL must be a bounded unprivileged http://127.0.0.1 URL with a unique path"
    )]
    InvalidSentinel,
    #[error("exact-session hostile configuration was refused: {0}")]
    ConfigCommit(String),
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

    #[cfg(test)]
    fn response_matching_after(
        &self,
        cursor: u64,
        session_id: u64,
        matches: impl Fn(&str) -> bool,
    ) -> Option<String> {
        self.routed_response_matching_after(cursor, &[session_id], matches)
            .map(|(_, response)| response)
    }

    fn routed_response_matching_after(
        &self,
        cursor: u64,
        session_ids: &[u64],
        matches: impl Fn(&str) -> bool,
    ) -> Option<(u64, String)> {
        let deadline = Instant::now() + RESPONSE_TIMEOUT;
        let mut events = self.events.lock().expect("event buffer poisoned");
        loop {
            if let Some(response) = events.iter().find_map(|event| {
                (event.sequence > cursor
                    && event
                        .session_id
                        .is_some_and(|session_id| session_ids.contains(&session_id)))
                .then(|| {
                    event
                        .response_json
                        .as_deref()
                        .filter(|response| matches(response))
                })
                .flatten()
                .and_then(|response| {
                    event
                        .session_id
                        .map(|session_id| (session_id, response.to_owned()))
                })
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

#[derive(Debug)]
struct UnavailableSettings;

impl NativeSettingsExecutor for UnavailableSettings {
    fn try_open(&self, _request: NativeSettingsRequest) -> NativeSettingsOpenResult {
        NativeSettingsOpenResult::Unavailable
    }
}

pub struct LinuxRunner {
    controller: Arc<RuntimeController>,
    observation: Arc<RuntimeObservation>,
    relay_observation: Arc<RuntimeRelayDiagnosticsObservation>,
    events: Arc<EventBuffer>,
    surfaces: BTreeMap<String, (u64, SurfaceLaunch)>,
    mode: RuntimeMode,
    state_path: std::path::PathBuf,
    next_surface_generation: u64,
}

impl std::fmt::Debug for LinuxRunner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LinuxRunner")
            .field("surfaces", &self.surfaces)
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
        let controller = RuntimeController::open_with_settings(
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
            Box::new(UnavailableSettings),
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
            surfaces: BTreeMap::new(),
            mode,
            state_path: runtime_root.join("uzel-state.json"),
            next_surface_generation: 0,
        };
        runner.restore_product_state()?;
        runner
            .persist_product_state()
            .map_err(StatePersistFailure::into_runner)?;
        Ok(runner)
    }

    pub fn mode(&self) -> RuntimeMode {
        self.mode
    }

    pub fn active_surfaces(&self) -> Vec<String> {
        self.surfaces.keys().cloned().collect()
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
            self.persist_product_state()
                .map_err(StatePersistFailure::into_runner)?;
            return Ok(handle.public_key);
        }

        let active = self.activate_account(handle.clone())?;
        if let Err(persist_error) = self.persist_product_state() {
            if matches!(&persist_error, StatePersistFailure::Unchanged(_)) {
                self.rollback_identity(previous_handle, handle)?;
            }
            return Err(persist_error.into_runner());
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

    fn persist_product_state(&self) -> Result<(), StatePersistFailure> {
        let state = ProductState {
            version: PRODUCT_STATE_VERSION,
            mode: self.mode.as_str().to_owned(),
            active_read_identity: self
                .get_read_identity()
                .map_err(|error| StatePersistFailure::Unchanged(error.to_string()))?,
            next_surface_generation: self.next_surface_generation,
        };
        let bytes = serde_json::to_vec(&state)
            .map_err(|error| StatePersistFailure::Unchanged(error.to_string()))?;
        if bytes.len() as u64 > MAXIMUM_PRODUCT_STATE_BYTES {
            return Err(StatePersistFailure::Unchanged(
                "state is oversized".to_owned(),
            ));
        }
        let temporary = self
            .state_path
            .with_extension(format!("json.{}.tmp", std::process::id()));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)
            .map_err(|error| StatePersistFailure::Unchanged(error.to_string()))?;
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| StatePersistFailure::Unchanged(error.to_string()))?;
        fs::rename(&temporary, &self.state_path)
            .map_err(|error| StatePersistFailure::Unchanged(error.to_string()))?;
        let parent = self.state_path.parent().ok_or_else(|| {
            StatePersistFailure::Replaced("state path has no parent directory".to_owned())
        })?;
        fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| StatePersistFailure::Replaced(error.to_string()))?;
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
            uncovered_authors: relay.uncovered_author_count,
            rejected_private_relays: relay.discovered_private_relays_rejected,
            sessions_rejected_over_cap: relay.sessions_rejected_over_cap,
            relay_details: relay
                .relays
                .into_iter()
                .map(|relay| RelayDiagnostic {
                    relay: relay.relay,
                    access: match relay.access {
                        RuntimeRelayAccess::Public => "public".to_owned(),
                        RuntimeRelayAccess::Nip42 { public_key } => {
                            format!("nip42:{}…", &public_key[..public_key.len().min(12)])
                        }
                    },
                    wire_subscriptions: relay.wire_subscription_count,
                    authors_served: relay.authors_served,
                    lanes: relay
                        .lanes
                        .into_iter()
                        .map(|lane| {
                            format!("{}:{}", relay_lane_name(lane.lane), lane.wire_subscriptions)
                        })
                        .collect(),
                    events_by_kind: relay
                        .events_by_kind
                        .into_iter()
                        .map(|kind| format!("{}:{}", kind.kind, kind.events))
                        .collect(),
                    nip11_freshness: relay.nip11_freshness,
                    nip11_last_error: relay.nip11_last_error,
                })
                .collect(),
            store_degraded: bounded_diagnostic(relay.store_degraded),
            transport_degraded: bounded_diagnostic(relay.transport_degraded),
        })
    }

    pub fn start_fixture(&mut self) -> Result<SurfaceLaunch, RunnerError> {
        self.start_named_fixture("good-morning")
    }

    pub fn start_named_fixture(&mut self, name: &str) -> Result<SurfaceLaunch, RunnerError> {
        let fixture = fixture_by_name(name).ok_or(RunnerError::UnknownFixture)?;
        if let Some((_, launch)) = self
            .surfaces
            .values()
            .find(|(_, launch)| launch.d_tag == fixture.d_tag)
        {
            return Ok(launch.clone());
        }
        if self.surfaces.len() == MAXIMUM_ACTIVE_FIXTURES {
            return Err(RunnerError::SurfaceCapacity);
        }
        self.next_surface_generation = self
            .next_surface_generation
            .checked_add(1)
            .ok_or(RunnerError::SurfaceGenerationExhausted)?;
        self.persist_product_state()
            .map_err(StatePersistFailure::into_runner)?;
        let verification = self.controller.verify_artifact(
            fixture.event.to_vec(),
            ArtifactCoordinate::Named {
                author: fixture.author.to_owned(),
                d_tag: fixture.d_tag.to_owned(),
            },
        );
        let artifact = verification.artifact.ok_or_else(|| {
            RunnerError::Verification(
                verification
                    .refusal
                    .map_or_else(|| "unknown refusal".to_owned(), |refusal| refusal.detail),
            )
        })?;
        if artifact.author() != fixture.author
            || artifact.d_tag().as_deref() != Some(fixture.d_tag)
            || artifact.aggregate_hash() != fixture.aggregate_hash
        {
            return Err(RunnerError::IdentityMismatch);
        }
        self.controller.install(Arc::clone(&artifact));
        for domain in fixture.domains {
            self.controller.set_grant(
                Arc::clone(&artifact),
                (*domain).to_owned(),
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
                session.author == fixture.author
                    && session.d_tag == fixture.d_tag
                    && session.aggregate_hash == fixture.aggregate_hash
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
        let surface_name = if fixture.name == "good-morning" {
            "surface-1"
        } else {
            fixture.name
        };
        let launch = SurfaceLaunch {
            surface_token: format!(
                "uzel-{}-generation-{}",
                surface_name, self.next_surface_generation
            ),
            artifact_base_url: fixture.artifact_base_url.to_owned(),
            artifact_html,
            title: fixture.title.to_owned(),
            author: fixture.author.to_owned(),
            d_tag: fixture.d_tag.to_owned(),
            aggregate_hash: fixture.aggregate_hash.to_owned(),
            domains: session.domains,
            unavailable_domains: session.unavailable_domains,
        };
        self.surfaces
            .insert(launch.surface_token.clone(), (session.id, launch.clone()));
        Ok(launch)
    }

    pub fn start_hostile_probe(
        &mut self,
        sentinel_url: &str,
    ) -> Result<SurfaceLaunch, RunnerError> {
        validate_sentinel_url(sentinel_url)?;
        let launch = self.start_named_fixture("hostile-egress")?;
        let (session_id, _) = self
            .surfaces
            .get(&launch.surface_token)
            .ok_or(RunnerError::SessionMissing)?;
        let commit = self.controller.commit_config_values(NativeConfigCommit {
            manifest_author: launch.author.clone(),
            d_tag: launch.d_tag.clone(),
            aggregate_hash: launch.aggregate_hash.clone(),
            session_id: *session_id,
            values_json: serde_json::json!({"sentinel": sentinel_url}).to_string(),
        });
        if !commit.accepted {
            let detail = commit
                .refusal
                .map_or_else(|| "unknown refusal".to_owned(), |refusal| refusal.detail);
            let _ = self.stop_fixture(&launch.surface_token);
            return Err(RunnerError::ConfigCommit(detail));
        }
        Ok(launch)
    }

    pub fn forward_from_surface(
        &self,
        surface_token: &str,
        envelope: &[u8],
    ) -> Result<RoutedEnvelope, RunnerError> {
        if envelope.len() > MAXIMUM_ENVELOPE_BYTES {
            return Err(RunnerError::EnvelopeTooLarge);
        }
        let (session_id, _) = self
            .surfaces
            .get(surface_token)
            .ok_or(RunnerError::UnknownSurface)?;
        let cursor = self.events.latest_sequence();
        let expectation = ResponseExpectation::from_envelope(envelope);
        self.controller
            .mapped_envelope(*session_id, envelope.to_vec());
        let eligible_sessions = if expectation.accepts_other_surface() {
            self.surfaces
                .values()
                .map(|(session_id, _)| *session_id)
                .collect::<Vec<_>>()
        } else {
            vec![*session_id]
        };
        let (target_session, envelope) = self
            .events
            .routed_response_matching_after(cursor, &eligible_sessions, |response| {
                expectation.matches(response)
            })
            .ok_or(RunnerError::ResponseTimeout)?;
        let surface_token = self
            .surfaces
            .iter()
            .find_map(|(surface_token, (session_id, _))| {
                (*session_id == target_session).then(|| surface_token.clone())
            })
            .ok_or(RunnerError::UnknownSurface)?;
        Ok(RoutedEnvelope {
            surface_token,
            envelope,
        })
    }

    pub fn stop_fixture(&mut self, surface_token: &str) -> Result<(), RunnerError> {
        let (session_id, _) = self
            .surfaces
            .remove(surface_token)
            .ok_or(RunnerError::UnknownSurface)?;
        self.controller.stop(session_id);
        Ok(())
    }
}

fn validate_sentinel_url(value: &str) -> Result<(), RunnerError> {
    if value.is_empty() || value.len() > 2_048 {
        return Err(RunnerError::InvalidSentinel);
    }
    let parsed = Url::parse(value).map_err(|_| RunnerError::InvalidSentinel)?;
    let port = parsed.port().ok_or(RunnerError::InvalidSentinel)?;
    if parsed.scheme() != "http"
        || parsed.host_str() != Some("127.0.0.1")
        || port < 1_024
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
        || parsed.path().len() <= 1
    {
        return Err(RunnerError::InvalidSentinel);
    }
    Ok(())
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
    AnyFromSource,
    IncEventAnySurface(String),
}

impl ResponseExpectation {
    fn from_envelope(envelope: &[u8]) -> Self {
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(envelope) else {
            return Self::AnyFromSource;
        };
        if let Some(id) = value.get("id").and_then(serde_json::Value::as_str) {
            return Self::Id(id.to_owned());
        }
        if value.get("type").and_then(serde_json::Value::as_str) == Some("shell.ready") {
            return Self::Type("shell.init");
        }
        if value.get("type").and_then(serde_json::Value::as_str) == Some("inc.emit")
            && let Some(topic) = value.get("topic").and_then(serde_json::Value::as_str)
        {
            return Self::IncEventAnySurface(topic.to_owned());
        }
        Self::AnyFromSource
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
            Self::IncEventAnySurface(topic) => {
                value.get("type").and_then(serde_json::Value::as_str) == Some("inc.event")
                    && value.get("topic").and_then(serde_json::Value::as_str)
                        == Some(topic.as_str())
            }
            Self::AnyFromSource => true,
        }
    }

    fn accepts_other_surface(&self) -> bool {
        matches!(self, Self::IncEventAnySurface(_))
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

    const GOOD_MORNING_AUTHOR: &str =
        "266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5";
    const LIVE_IDENTITY: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";

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
            serde_json::from_str::<Value>(&response.envelope).unwrap()["type"],
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
        serde_json::from_str(&response.envelope).unwrap()
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
    fn inc_emit_waits_for_an_inc_event_not_an_unrelated_push() {
        let expectation = ResponseExpectation::from_envelope(
            br#"{"type":"inc.emit","topic":"napplet:profile/open","payload":{}}"#,
        );
        assert!(expectation.accepts_other_surface());
        assert!(!expectation.matches(r#"{"type":"identity.changed"}"#));
        assert!(!expectation.matches(r#"{"type":"inc.emit.result","ok":true}"#));
        assert!(!expectation.matches(r#"{"type":"inc.event","topic":"other"}"#));
        assert!(expectation.matches(
            r#"{"type":"inc.event","topic":"napplet:profile/open","sender":"follow-list"}"#
        ));
    }

    #[test]
    fn verified_fixture_handshakes_through_runtime() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let launch = runner.start_fixture().unwrap();
        assert_eq!(
            launch.aggregate_hash,
            fixture_by_name("good-morning").unwrap().aggregate_hash
        );
        assert_eq!(launch.domains, ["identity", "inc", "outbox", "shell"]);
        assert_eq!(launch.unavailable_domains, ["link", "resource", "theme"]);

        let response = runner
            .forward_from_surface(&launch.surface_token, br#"{"type":"shell.ready"}"#)
            .unwrap();
        let response: Value = serde_json::from_str(&response.envelope).unwrap();
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
    fn hostile_probe_commits_exact_session_config_before_returning() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let sentinel = "http://127.0.0.1:43129/uzel-hostile/run-7";
        let launch = runner.start_hostile_probe(sentinel).unwrap();

        let shell = runner
            .forward_from_surface(&launch.surface_token, br#"{"type":"shell.ready"}"#)
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&shell.envelope).unwrap()["type"],
            "shell.init"
        );
        let config = runner
            .forward_from_surface(
                &launch.surface_token,
                br#"{"type":"config.get","id":"slice-06-config"}"#,
            )
            .unwrap();
        let config: Value = serde_json::from_str(&config.envelope).unwrap();
        assert_eq!(config["type"], "config.values");
        assert_eq!(config["values"]["sentinel"], sentinel);
    }

    #[test]
    fn hostile_probe_rejects_non_loopback_or_ambiguous_sentinels() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        for sentinel in [
            "",
            "http://127.0.0.1:9/probe",
            "https://127.0.0.1:43129/probe",
            "http://localhost:43129/probe",
            "http://user@127.0.0.1:43129/probe",
            "http://127.0.0.1:43129/",
            "http://127.0.0.1:43129/probe#fragment",
        ] {
            assert!(matches!(
                runner.start_hostile_probe(sentinel),
                Err(RunnerError::InvalidSentinel)
            ));
        }
        assert!(runner.active_surfaces().is_empty());
    }

    #[test]
    fn nmp_parses_and_persists_the_active_read_identity() {
        let temp = TempDir::new().unwrap();
        {
            let mut runner = LinuxRunner::open(temp.path()).unwrap();
            assert_eq!(runner.get_read_identity().unwrap(), None);
            assert_eq!(
                runner
                    .set_read_identity(GOOD_MORNING_AUTHOR.to_owned())
                    .unwrap(),
                GOOD_MORNING_AUTHOR
            );
            assert!(matches!(
                runner.set_read_identity(GOOD_MORNING_AUTHOR.to_ascii_uppercase()),
                Err(RunnerError::Identity(_))
            ));
        }
        let runner = LinuxRunner::open(temp.path()).unwrap();
        assert_eq!(
            runner.get_read_identity().unwrap().as_deref(),
            Some(GOOD_MORNING_AUTHOR)
        );
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
    fn stopped_fixture_relaunches_with_a_new_session_and_surface() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let first = runner.start_named_fixture("follow-list").unwrap();
        let first_session = runner.surfaces[&first.surface_token].0;

        runner.stop_fixture(&first.surface_token).unwrap();
        let second = runner.start_named_fixture("follow-list").unwrap();
        let second_session = runner.surfaces[&second.surface_token].0;

        assert_ne!(first.surface_token, second.surface_token);
        assert_ne!(first_session, second_session);
        assert!(matches!(
            runner.forward_from_surface(&first.surface_token, br#"{"type":"shell.ready"}"#),
            Err(RunnerError::UnknownSurface)
        ));
    }

    #[test]
    fn identity_activation_rolls_back_when_product_state_cannot_persist() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        assert_eq!(
            runner
                .set_read_identity(GOOD_MORNING_AUTHOR.to_owned())
                .unwrap(),
            GOOD_MORNING_AUTHOR
        );
        let temporary = temp
            .path()
            .join(format!("uzel-state.json.{}.tmp", std::process::id()));
        fs::write(&temporary, b"occupied").unwrap();

        assert!(matches!(
            runner.set_read_identity(LIVE_IDENTITY.to_owned()),
            Err(RunnerError::StatePersist(_))
        ));
        assert_eq!(
            runner.get_read_identity().unwrap().as_deref(),
            Some(GOOD_MORNING_AUTHOR)
        );
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
        for name in ["follow-list", "profile-card", "hostile-egress"] {
            let fixture = fixture_by_name(name).unwrap();
            let temp = TempDir::new().unwrap();
            let controller = RuntimeController::open_with_settings(
                RuntimeConfig {
                    runtime_store_path: temp.path().join("runtime.sqlite3").display().to_string(),
                    nmp_store_path: None,
                    artifact_cache_path: temp.path().join("artifacts").display().to_string(),
                    ..RuntimeConfig::default()
                },
                Box::new(ExactFixtureSource),
                Box::new(AcceptSettings),
            )
            .unwrap();
            let events = Arc::new(EventBuffer::default());
            let observation =
                Arc::clone(&controller).observe(Box::new(EventSink(Arc::clone(&events))));
            let _observation = observation.observation.expect("runtime observation starts");
            let verification = controller.verify_artifact(
                fixture.event.to_vec(),
                ArtifactCoordinate::Named {
                    author: fixture.author.to_owned(),
                    d_tag: fixture.d_tag.to_owned(),
                },
            );
            let artifact = verification.artifact.unwrap_or_else(|| {
                panic!(
                    "signed fixture {} verifies: {:?}",
                    fixture.d_tag, verification.refusal
                )
            });
            assert!(verification.refusal.is_none());
            assert_eq!(artifact.author(), fixture.author);
            assert_eq!(artifact.d_tag().as_deref(), Some(fixture.d_tag));
            assert_eq!(artifact.aggregate_hash(), fixture.aggregate_hash);

            controller.install(Arc::clone(&artifact));
            for domain in fixture.domains {
                controller.set_grant(
                    Arc::clone(&artifact),
                    (*domain).to_owned(),
                    RuntimeSensitivity::Ordinary,
                    RuntimeGrantDecision::AllowExactBuild,
                );
            }
            controller.launch(artifact, RuntimeExecutionProfile::Legacy);
            let session = match controller.snapshot() {
                RuntimeSnapshotProjection::Snapshot { snapshot } => snapshot
                    .sessions
                    .into_iter()
                    .find(|session| session.d_tag == fixture.d_tag)
                    .expect("fixture session exists"),
                RuntimeSnapshotProjection::Refused { refusal, .. } => {
                    panic!("snapshot refused: {}", refusal.detail)
                }
            };
            for domain in fixture.domains {
                assert!(session.domains.iter().any(|granted| granted == *domain));
            }
            if fixture.d_tag == "egress-probe" {
                let cursor = events.latest_sequence();
                controller.mapped_envelope(session.id, br#"{"type":"shell.ready"}"#.to_vec());
                let response = events
                    .response_after(cursor, session.id)
                    .expect("NAP-SHELL initialization response");
                let response: Value = serde_json::from_str(&response).unwrap();
                assert_eq!(response["type"], "shell.init");
                let sentinel = "http://127.0.0.1:43129/hostile-egress?run=fixture";
                let commit = controller.commit_config_values(NativeConfigCommit {
                    manifest_author: fixture.author.to_owned(),
                    d_tag: fixture.d_tag.to_owned(),
                    aggregate_hash: fixture.aggregate_hash.to_owned(),
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
                VerifiedRead::Bytes { bytes, .. } => assert_eq!(bytes, fixture.index),
                VerifiedRead::Refused { refusal } => panic!("verified read refused: {refusal:?}"),
            }
            controller.close();
        }
    }

    #[test]
    fn profile_open_crosses_inc_with_runtime_owned_sender() {
        let temp = TempDir::new().unwrap();
        let mut runner = LinuxRunner::open(temp.path()).unwrap();
        let follow = runner.start_named_fixture("follow-list").unwrap();
        let profile = runner.start_named_fixture("profile-card").unwrap();
        for launch in [&follow, &profile] {
            let response = runner
                .forward_from_surface(&launch.surface_token, br#"{"type":"shell.ready"}"#)
                .unwrap();
            assert_eq!(response.surface_token, launch.surface_token);
            let response: Value = serde_json::from_str(&response.envelope).unwrap();
            assert_eq!(response["type"], "shell.init");
        }

        let response = runner
            .forward_from_surface(
                &profile.surface_token,
                br#"{"type":"inc.subscribe","id":"profile-open-sub","topic":"napplet:profile/open"}"#,
            )
            .unwrap();
        assert_eq!(response.surface_token, profile.surface_token);
        let response: Value = serde_json::from_str(&response.envelope).unwrap();
        assert_eq!(response["type"], "inc.subscribe.result");

        let delivery = runner
            .forward_from_surface(
                &follow.surface_token,
                br#"{"type":"inc.emit","topic":"napplet:profile/open","payload":{"version":1,"pubkey":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#,
            )
            .unwrap();
        assert_eq!(delivery.surface_token, profile.surface_token);
        let delivery: Value = serde_json::from_str(&delivery.envelope).unwrap();
        assert_eq!(delivery["type"], "inc.event");
        assert_eq!(delivery["topic"], "napplet:profile/open");
        assert_eq!(delivery["sender"], "follow-list");
        assert_eq!(delivery["payload"]["version"], 1);
        assert_eq!(delivery["payload"]["pubkey"], "a".repeat(64));
    }
}
