use std::{
    io::ErrorKind,
    net::{TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use serde::{Deserialize, Serialize};

const ACCEPT_POLL: Duration = Duration::from_millis(10);
const PROBE_SETTLE: Duration = Duration::from_millis(250);
const MAXIMUM_SURFACE_TOKEN_BYTES: usize = 128;

#[derive(Debug, Default)]
pub struct HostileProbeState {
    next_run: AtomicU64,
    native_calls: AtomicUsize,
    active: Mutex<Option<LiveProbe>>,
}

#[derive(Debug)]
struct LiveProbe {
    sentinel_url: String,
    surface_token: Option<String>,
    accepts: Arc<AtomicUsize>,
    accept_error: Arc<Mutex<Option<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for LiveProbe {
    fn drop(&mut self) {
        let _ = self.stop_and_drain();
    }
}

impl LiveProbe {
    fn stop_and_drain(&mut self) -> Result<(), String> {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| "hostile sentinel thread panicked".to_owned())?;
        }
        let mut accept_error = self
            .accept_error
            .lock()
            .map_err(|_| "hostile sentinel error state is poisoned".to_owned())?;
        if let Some(error) = accept_error.take() {
            return Err(format!("hostile sentinel accept loop failed: {error}"));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostileProbeReport {
    fetch: bool,
    xhr: bool,
    websocket: bool,
    eventsource: bool,
    image: bool,
    worker: bool,
    service_worker: bool,
    beacon: BeaconAttempt,
    media: bool,
    iframe: bool,
    form: bool,
    navigation: bool,
    popup: bool,
    tauri_internals: bool,
    tauri_global: bool,
    wry_ipc: bool,
    parent_readable: bool,
    raw_webkit_transport: bool,
    raw_invoke_attempted: bool,
    identity_mutation_api: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BeaconAttempt {
    Queued,
    Rejected,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostileProbeVerdict {
    pub network_denials: usize,
    pub sentinel_accepts: usize,
    pub native_calls: usize,
}

impl HostileProbeState {
    pub fn begin(&self) -> Result<String, String> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| "hostile probe state is poisoned".to_owned())?;
        if active.is_some() {
            return Err("a hostile probe is already active".to_owned());
        }
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .map_err(|error| format!("hostile sentinel could not bind: {error}"))?;
        let address = listener
            .local_addr()
            .map_err(|error| format!("hostile sentinel address failed: {error}"))?;
        let control = TcpStream::connect(address)
            .map_err(|error| format!("hostile sentinel control connection failed: {error}"))?;
        let (accepted_control, peer) = listener
            .accept()
            .map_err(|error| format!("hostile sentinel control accept failed: {error}"))?;
        if !peer.ip().is_loopback() {
            return Err("hostile sentinel control peer was not loopback".to_owned());
        }
        drop(accepted_control);
        drop(control);
        listener
            .set_nonblocking(true)
            .map_err(|error| format!("hostile sentinel nonblocking mode failed: {error}"))?;

        let run = self
            .next_run
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                current.checked_add(1)
            })
            .map_err(|_| "hostile probe run counter is exhausted".to_owned())?
            + 1;
        let sentinel_url = format!(
            "http://127.0.0.1:{}/uzel-hostile/{}-{run}",
            address.port(),
            std::process::id()
        );
        let accepts = Arc::new(AtomicUsize::new(0));
        let accept_error = Arc::new(Mutex::new(None));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_accepts = Arc::clone(&accepts);
        let thread_accept_error = Arc::clone(&accept_error);
        let thread_stop = Arc::clone(&stop);
        let thread = thread::Builder::new()
            .name("uzel-hostile-sentinel".to_owned())
            .spawn(move || {
                loop {
                    match listener.accept() {
                        Ok((stream, _)) => {
                            thread_accepts.fetch_add(1, Ordering::AcqRel);
                            drop(stream);
                        }
                        Err(error) if error.kind() == ErrorKind::WouldBlock => {
                            if thread_stop.load(Ordering::Acquire) {
                                break;
                            }
                            thread::sleep(ACCEPT_POLL);
                        }
                        Err(error) => {
                            if let Ok(mut stored) = thread_accept_error.lock() {
                                *stored = Some(error.to_string());
                            }
                            break;
                        }
                    }
                }
            })
            .map_err(|error| format!("hostile sentinel thread failed: {error}"))?;
        self.native_calls.store(0, Ordering::Release);
        *active = Some(LiveProbe {
            sentinel_url: sentinel_url.clone(),
            surface_token: None,
            accepts,
            accept_error,
            stop,
            thread: Some(thread),
        });
        Ok(sentinel_url)
    }

    pub fn attach(&self, sentinel_url: &str, surface_token: &str) -> Result<(), String> {
        if !valid_surface_token(surface_token) {
            return Err("hostile surface token is invalid".to_owned());
        }
        let mut active = self
            .active
            .lock()
            .map_err(|_| "hostile probe state is poisoned".to_owned())?;
        let probe = active
            .as_mut()
            .ok_or_else(|| "no hostile probe is active".to_owned())?;
        if probe.sentinel_url != sentinel_url || probe.surface_token.is_some() {
            return Err("hostile probe reservation does not match".to_owned());
        }
        probe.surface_token = Some(surface_token.to_owned());
        Ok(())
    }

    pub fn cancel(&self) {
        if let Ok(mut active) = self.active.lock() {
            active.take();
        }
    }

    pub fn record_native_call(&self) {
        self.native_calls.fetch_add(1, Ordering::AcqRel);
    }

    pub fn finish(
        &self,
        surface_token: &str,
        report: HostileProbeReport,
    ) -> Result<HostileProbeVerdict, String> {
        let mut probe = {
            let mut active = self
                .active
                .lock()
                .map_err(|_| "hostile probe state is poisoned".to_owned())?;
            let probe = active
                .as_ref()
                .ok_or_else(|| "no hostile probe is active".to_owned())?;
            if probe.surface_token.as_deref() != Some(surface_token) {
                return Err("hostile probe surface token does not match".to_owned());
            }
            active.take().expect("active probe checked above")
        };
        thread::sleep(PROBE_SETTLE);
        probe.stop_and_drain()?;
        let sentinel_accepts = probe.accepts.load(Ordering::Acquire);
        let native_calls = self.native_calls.load(Ordering::Acquire);
        let denials = report.network_denials();
        report.validate_native_boundary()?;
        if sentinel_accepts != 0 {
            return Err(format!(
                "hostile sentinel accepted {sentinel_accepts} probe connections"
            ));
        }
        if native_calls != 0 {
            return Err(format!(
                "raw hostile IPC executed {native_calls} authenticated native commands"
            ));
        }
        Ok(HostileProbeVerdict {
            network_denials: denials,
            sentinel_accepts,
            native_calls,
        })
    }
}

impl HostileProbeReport {
    fn failed_network_probes(&self) -> Vec<&'static str> {
        [
            ("fetch", self.fetch),
            ("xhr", self.xhr),
            ("websocket", self.websocket),
            ("eventsource", self.eventsource),
            ("image", self.image),
            ("worker", self.worker),
            ("serviceWorker", self.service_worker),
            ("media", self.media),
            ("iframe", self.iframe),
            ("form", self.form),
            ("navigation", self.navigation),
            ("popup", self.popup),
        ]
        .into_iter()
        .filter_map(|(name, denied)| (!denied).then_some(name))
        .collect()
    }

    fn network_denials(&self) -> usize {
        let denials = [
            self.fetch,
            self.xhr,
            self.websocket,
            self.eventsource,
            self.image,
            self.worker,
            self.service_worker,
            self.media,
            self.iframe,
            self.form,
            self.navigation,
            self.popup,
        ];
        // sendBeacon queue acceptance does not prove transport. Deserializing either
        // explicit outcome proves the API call completed; the live sentinel decides egress.
        let beacon_attempted =
            matches!(self.beacon, BeaconAttempt::Queued | BeaconAttempt::Rejected);
        denials.into_iter().filter(|denied| *denied).count() + usize::from(beacon_attempted)
    }

    fn validate_native_boundary(&self) -> Result<(), String> {
        const EXPECTED_NETWORK_DENIALS: usize = 13;
        if self.network_denials() != EXPECTED_NETWORK_DENIALS {
            return Err(format!(
                "hostile browser network probes reported success: {}",
                self.failed_network_probes().join(",")
            ));
        }
        if self.tauri_internals
            || self.tauri_global
            || self.wry_ipc
            || self.parent_readable
            || self.identity_mutation_api
        {
            return Err("sandboxed hostile child retained trusted authority".to_owned());
        }
        if !self.raw_webkit_transport || !self.raw_invoke_attempted {
            return Err("raw WebKit transport denial was not actually attempted".to_owned());
        }
        Ok(())
    }
}

fn valid_surface_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAXIMUM_SURFACE_TOKEN_BYTES
        && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    fn port(url: &str) -> u16 {
        url.strip_prefix("http://127.0.0.1:")
            .unwrap()
            .split('/')
            .next()
            .unwrap()
            .parse()
            .unwrap()
    }

    fn accepted_report() -> HostileProbeReport {
        HostileProbeReport {
            fetch: true,
            xhr: true,
            websocket: true,
            eventsource: true,
            image: true,
            worker: true,
            service_worker: true,
            beacon: BeaconAttempt::Queued,
            media: true,
            iframe: true,
            form: true,
            navigation: true,
            popup: true,
            tauri_internals: false,
            tauri_global: false,
            wry_ipc: false,
            parent_readable: false,
            raw_webkit_transport: true,
            raw_invoke_attempted: true,
            identity_mutation_api: false,
        }
    }

    #[test]
    fn control_accept_is_not_counted_as_a_probe_connection() {
        let state = HostileProbeState::default();
        let url = state.begin().unwrap();
        state.attach(&url, "hostile-surface").unwrap();
        let probe = state.active.lock().unwrap().take().unwrap();
        assert_eq!(probe.accepts.load(Ordering::Acquire), 0);
    }

    #[test]
    fn later_loopback_connection_is_counted_separately() {
        let state = HostileProbeState::default();
        let url = state.begin().unwrap();
        state.attach(&url, "hostile-surface").unwrap();
        let stream = TcpStream::connect(("127.0.0.1", port(&url))).unwrap();
        drop(stream);
        let deadline = Instant::now() + Duration::from_secs(1);
        while state
            .active
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .accepts
            .load(Ordering::Acquire)
            == 0
        {
            assert!(
                Instant::now() < deadline,
                "probe connection was not counted"
            );
            thread::sleep(ACCEPT_POLL);
        }
        let probe = state.active.lock().unwrap().take().unwrap();
        assert_eq!(probe.accepts.load(Ordering::Acquire), 1);
    }

    #[test]
    fn queued_beacon_is_counted_only_with_the_external_sentinel() {
        let report = accepted_report();
        assert_eq!(report.network_denials(), 13);
        assert!(report.failed_network_probes().is_empty());
    }

    #[test]
    fn recorded_accept_loop_failure_rejects_finalization() {
        let state = HostileProbeState::default();
        let url = state.begin().unwrap();
        state.attach(&url, "hostile-surface").unwrap();
        let accept_error = Arc::clone(&state.active.lock().unwrap().as_ref().unwrap().accept_error);
        *accept_error.lock().unwrap() = Some("injected accept failure".to_owned());

        let error = state
            .finish("hostile-surface", accepted_report())
            .unwrap_err();
        assert!(error.contains("accept loop failed: injected accept failure"));
    }
}
