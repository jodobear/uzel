#![forbid(unsafe_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::PathBuf,
};

use napd_protocol::{
    ClientError, Diagnostics, FetchedSurface, NappletReview, Request, Response, RoutedEnvelope,
    UnixClient, VERSION,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tauri::Manager;

mod hostile_probe;

use hostile_probe::{HostileProbeReport, HostileProbeState, HostileProbeVerdict};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SurfaceLaunch {
    surface_token: String,
    artifact_base_url: String,
    artifact_html: String,
    title: String,
    author: String,
    d_tag: String,
    aggregate_hash: String,
    artifact_digest: String,
    domains: Vec<String>,
    unavailable_domains: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfirmNappletError {
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    surface_token: Option<String>,
    detail: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReviewNappletError {
    kind: &'static str,
    detail: String,
}

impl From<ClientError> for ReviewNappletError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::AmbiguousOperation(error) => Self {
                kind: "reviewAmbiguous",
                detail: error.to_string(),
            },
            error => Self {
                kind: "refused",
                detail: error.to_string(),
            },
        }
    }
}

impl From<ClientError> for ConfirmNappletError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::TransferCleanupFailed {
                surface_token,
                transfer_error,
                cleanup_error,
            } => Self {
                kind: "cleanupRequired",
                surface_token: Some(surface_token),
                detail: format!(
                    "asset transfer failed ({transfer_error}); cleanup also failed ({cleanup_error})"
                ),
            },
            ClientError::AmbiguousOperation(error) => Self {
                kind: "confirmationAmbiguous",
                surface_token: None,
                detail: error.to_string(),
            },
            error => Self {
                kind: "refused",
                surface_token: None,
                detail: error.to_string(),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeStatus {
    mode: String,
    active_surfaces: Vec<String>,
    pending_reviews: Vec<String>,
    active_identity: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeCleanupFailure {
    kind: &'static str,
    token: String,
    detail: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeReconciliation {
    runtime: RuntimeStatus,
    cleanup_failures: Vec<RuntimeCleanupFailure>,
}

fn read_runtime_status(client: &UnixClient) -> Result<RuntimeStatus, String> {
    match client
        .request(&Request::Status)
        .map_err(|error| error.to_string())?
    {
        Response::Status {
            mode,
            active_surfaces,
            pending_reviews,
            active_identity,
            ..
        } => Ok(RuntimeStatus {
            mode,
            active_surfaces,
            pending_reviews,
            active_identity,
        }),
        _ => Err("daemon returned an unexpected status response".to_owned()),
    }
}

#[tauri::command]
fn runtime_status(client: tauri::State<'_, UnixClient>) -> Result<RuntimeStatus, String> {
    read_runtime_status(&client)
}

fn clean_token_snapshot(
    tokens: &[String],
    mut clean: impl FnMut(&str) -> Result<(), String>,
) -> BTreeMap<String, String> {
    let mut failures = BTreeMap::new();
    for token in tokens.iter().collect::<BTreeSet<_>>() {
        if let Err(error) = clean(token) {
            failures.insert(token.clone(), error);
        }
    }
    failures
}

fn remaining_cleanup_failures(
    kind: &'static str,
    tokens: &[String],
    attempted_failures: &BTreeMap<String, String>,
    default_detail: &'static str,
) -> Vec<RuntimeCleanupFailure> {
    tokens
        .iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|token| RuntimeCleanupFailure {
            kind,
            token: token.clone(),
            detail: attempted_failures
                .get(token)
                .cloned()
                .unwrap_or_else(|| default_detail.to_owned()),
        })
        .collect()
}

#[tauri::command]
fn reconcile_runtime(
    client: tauri::State<'_, UnixClient>,
    hostile_state: tauri::State<'_, HostileProbeState>,
) -> Result<RuntimeReconciliation, String> {
    // A fresh renderer owns none of the daemon's existing surfaces or reviews.
    // Clean each exact initial snapshot, then use a second status read as the
    // authoritative result for ambiguous replies.
    hostile_state.cancel();
    let before = read_runtime_status(&client)?;
    let surface_failures = clean_token_snapshot(&before.active_surfaces, |surface_token| {
        client
            .stop_fixture(surface_token)
            .map_err(|error| error.to_string())
    });
    let review_failures = clean_token_snapshot(&before.pending_reviews, |token| {
        client
            .cancel_napplet_review(token)
            .map_err(|error| error.to_string())
    });
    let runtime = read_runtime_status(&client)?;
    let mut cleanup_failures = remaining_cleanup_failures(
        "surface",
        &runtime.active_surfaces,
        &surface_failures,
        "daemon still reports the surface after cleanup",
    );
    cleanup_failures.extend(remaining_cleanup_failures(
        "review",
        &runtime.pending_reviews,
        &review_failures,
        "daemon still reports the review after cancellation",
    ));
    if runtime.active_surfaces.is_empty()
        && runtime.pending_reviews.is_empty()
        && cleanup_failures.is_empty()
    {
        client.retire_catalog_operations();
    }
    Ok(RuntimeReconciliation {
        runtime,
        cleanup_failures,
    })
}

#[tauri::command]
fn select_read_identity(
    client: tauri::State<'_, UnixClient>,
    public_identity: String,
) -> Result<String, String> {
    match client
        .request(&Request::SetReadIdentity { public_identity })
        .map_err(|error| error.to_string())?
    {
        Response::Identity {
            active_public_key: Some(active_public_key),
        } => Ok(active_public_key),
        _ => Err("daemon returned no active read identity".to_owned()),
    }
}

#[tauri::command]
fn runtime_diagnostics(client: tauri::State<'_, UnixClient>) -> Result<Diagnostics, String> {
    match client
        .request(&Request::Diagnostics)
        .map_err(|error| error.to_string())?
    {
        Response::Diagnostics { diagnostics } => Ok(diagnostics),
        _ => Err("daemon returned an unexpected diagnostics response".to_owned()),
    }
}

#[tauri::command]
fn start_fixture(
    client: tauri::State<'_, UnixClient>,
    fixture: String,
) -> Result<SurfaceLaunch, String> {
    let fetched = client
        .start_named_fixture(&fixture)
        .map_err(|error| error.to_string())?;
    println!(
        "UZEL_FIXTURE_VERIFIED fixture={} aggregate={}",
        fixture, fetched.surface.aggregate_hash
    );
    project_surface(fetched)
}

#[tauri::command]
fn review_napplet(
    client: tauri::State<'_, UnixClient>,
    coordinate: String,
) -> Result<NappletReview, ReviewNappletError> {
    client
        .review_napplet(&coordinate)
        .map_err(ReviewNappletError::from)
}

#[tauri::command]
fn cancel_napplet_review(
    client: tauri::State<'_, UnixClient>,
    token: String,
) -> Result<(), String> {
    client
        .cancel_napplet_review(&token)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn confirm_napplet(
    client: tauri::State<'_, UnixClient>,
    token: String,
    expected_author: String,
    expected_d_tag: String,
    expected_aggregate_hash: String,
    granted_domains: Vec<String>,
) -> Result<SurfaceLaunch, ConfirmNappletError> {
    let fetched = client
        .confirm_napplet(
            &token,
            &expected_author,
            &expected_d_tag,
            &expected_aggregate_hash,
            granted_domains,
        )
        .map_err(ConfirmNappletError::from)?;
    println!(
        "UZEL_CATALOG_VERIFIED author={} d_tag={} aggregate={}",
        fetched.surface.author, fetched.surface.d_tag, fetched.surface.aggregate_hash
    );
    project_surface(fetched).map_err(|detail| ConfirmNappletError {
        kind: "refused",
        surface_token: None,
        detail,
    })
}

fn project_surface(fetched: FetchedSurface) -> Result<SurfaceLaunch, String> {
    let artifact_digest = format!("{:x}", Sha256::digest(&fetched.artifact_bytes));
    let artifact_html = String::from_utf8(fetched.artifact_bytes)
        .map_err(|error| format!("verified artifact is not UTF-8: {error}"))?;
    Ok(SurfaceLaunch {
        surface_token: fetched.surface.surface_token,
        artifact_base_url: fetched.surface.artifact_base_url,
        artifact_html,
        title: fetched.surface.title,
        author: fetched.surface.author,
        d_tag: fetched.surface.d_tag,
        aggregate_hash: fetched.surface.aggregate_hash,
        artifact_digest,
        domains: fetched.surface.domains,
        unavailable_domains: fetched.surface.unavailable_domains,
    })
}

#[tauri::command]
fn hostile_probe_enabled() -> bool {
    env::var("UZEL_RUN_HOSTILE_PROBE").as_deref() == Ok("1")
}

#[tauri::command]
fn webkit_recovery_probe_enabled() -> bool {
    env::var("UZEL_RUN_WEBKIT_RECOVERY_PROBE").as_deref() == Ok("1")
}

#[tauri::command]
fn start_hostile_probe(
    client: tauri::State<'_, UnixClient>,
    state: tauri::State<'_, HostileProbeState>,
) -> Result<SurfaceLaunch, String> {
    let sentinel_url = state.begin()?;
    let fetched = match client.start_hostile_probe(&sentinel_url) {
        Ok(fetched) => fetched,
        Err(error) => {
            state.cancel();
            return Err(error.to_string());
        }
    };
    if let Err(error) = state.attach(&sentinel_url, &fetched.surface.surface_token) {
        state.cancel();
        let _ = client.stop_fixture(&fetched.surface.surface_token);
        return Err(error);
    }
    println!(
        "UZEL_FIXTURE_VERIFIED fixture=hostile-egress aggregate={}",
        fetched.surface.aggregate_hash
    );
    println!(
        "UZEL_HOSTILE_SENTINEL_READY control=accepted surface={} url={}",
        fetched.surface.surface_token, sentinel_url
    );
    project_surface(fetched)
}

#[tauri::command]
fn stop_fixture(
    client: tauri::State<'_, UnixClient>,
    hostile_state: tauri::State<'_, HostileProbeState>,
    surface_token: String,
) -> Result<(), String> {
    hostile_state.cancel_surface(&surface_token)?;
    client
        .stop_fixture(&surface_token)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn forward_surface_envelope(
    client: tauri::State<'_, UnixClient>,
    surface_token: String,
    envelope: String,
) -> Result<RoutedEnvelope, String> {
    let response = client
        .request(&Request::ForwardEnvelope {
            surface_token: surface_token.clone(),
            envelope,
        })
        .map_err(|error| error.to_string())?;
    let Response::Envelope {
        surface_token: target_surface,
        envelope,
    } = response
    else {
        return Err("daemon returned an unexpected envelope response".to_owned());
    };
    let response_type = serde_json::from_str::<serde_json::Value>(&envelope)
        .ok()
        .and_then(|value| value["type"].as_str().map(str::to_owned))
        .unwrap_or_default();
    match response_type.as_str() {
        "shell.init" => println!("UZEL_NAP_SHELL_OK surface={target_surface}"),
        "identity.getPublicKey.result" | "identity.getFollows.result" => {
            println!("UZEL_ARTIFACT_RESPONDED type={response_type}")
        }
        "inc.event" => println!("UZEL_INC_ROUTED target={target_surface}"),
        _ => {}
    }
    Ok(RoutedEnvelope {
        surface_token: target_surface,
        envelope,
    })
}

#[tauri::command]
fn report_shell_accepted(surface_token: String) -> Result<(), String> {
    if surface_token.is_empty()
        || surface_token.len() > 128
        || surface_token.chars().any(char::is_control)
    {
        return Err("shell acceptance surface token is invalid".to_owned());
    }
    println!("UZEL_SHELL_ACCEPTED surface={surface_token}");
    Ok(())
}

#[tauri::command]
fn report_webkit_recovery(
    client: tauri::State<'_, UnixClient>,
    before: Vec<String>,
    after: Vec<String>,
) -> Result<(), String> {
    let before = before.into_iter().collect::<BTreeSet<_>>();
    let after = after.into_iter().collect::<BTreeSet<_>>();
    if before.len() != 2 || after.len() != 2 || !before.is_disjoint(&after) {
        return Err("WebKit recovery surface sets are invalid".to_owned());
    }
    let status = read_runtime_status(&client)?;
    let active = status.active_surfaces.into_iter().collect::<BTreeSet<_>>();
    if !before.is_disjoint(&active) || !after.is_subset(&active) {
        return Err("WebKit recovery did not replace both trusted surfaces".to_owned());
    }
    println!(
        "UZEL_WEBKIT_RECOVERY_OK before={} after={} source_bound=true",
        before.into_iter().collect::<Vec<_>>().join(","),
        after.into_iter().collect::<Vec<_>>().join(",")
    );
    Ok(())
}

#[tauri::command]
fn hostile_native_probe(state: tauri::State<'_, HostileProbeState>) -> Result<(), String> {
    state.record_native_call();
    Err("hostile native probe reached authenticated command dispatch".to_owned())
}

#[tauri::command]
fn finish_hostile_probe(
    client: tauri::State<'_, UnixClient>,
    state: tauri::State<'_, HostileProbeState>,
    surface_token: String,
    report: HostileProbeReport,
) -> Result<HostileProbeVerdict, String> {
    println!("UZEL_HOSTILE_RESULT_RECEIVED surface={surface_token}");
    let verdict = state.finish(&surface_token, report);
    let stopped = client
        .request(&Request::StopFixture {
            surface_token: surface_token.clone(),
        })
        .map_err(|error| error.to_string())
        .and_then(|response| match response {
            Response::Stopped => Ok(()),
            _ => Err("daemon returned an unexpected hostile stop response".to_owned()),
        });
    let verdict = match verdict {
        Ok(verdict) => verdict,
        Err(error) => {
            eprintln!("UZEL_HOSTILE_PROBE_FAILED surface={surface_token} reason={error}");
            return Err(error);
        }
    };
    stopped?;
    println!(
        "UZEL_HOSTILE_PROBE_OK surface={} network_denials={} sentinel_accepts={} native_calls={} source_bound=true",
        surface_token, verdict.network_denials, verdict.sentinel_accepts, verdict.native_calls
    );
    Ok(verdict)
}

#[tauri::command]
fn report_user_mode(diagnostics_hidden: bool, unsafe_controls_absent: bool) -> Result<(), String> {
    if !diagnostics_hidden || !unsafe_controls_absent {
        return Err("user mode exposed diagnostics or unsafe fixture controls".to_owned());
    }
    println!("UZEL_USER_MODE_OK diagnostics=hidden unsafe_controls=absent");
    Ok(())
}

fn navigation_policy() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("navigation-policy")
        .on_navigation(|_webview, url| {
            let allowed = allowed_navigation(url);
            if !allowed {
                eprintln!(
                    "UZEL_NAVIGATION_DENIED scheme={} host={}",
                    url.scheme(),
                    url.host_str().unwrap_or("none")
                );
            }
            allowed
        })
        .build()
}

fn allowed_navigation(url: &tauri::Url) -> bool {
    if exact_outer_navigation(url) {
        return true;
    }
    match url.scheme() {
        "tauri" => true,
        "about" => matches!(url.path(), "blank" | "srcdoc"),
        "http" | "https" if url.host_str() == Some("tauri.localhost") => true,
        "http"
            if cfg!(debug_assertions)
                && url.host_str() == Some("127.0.0.1")
                && url.port() == Some(1420) =>
        {
            true
        }
        _ => false,
    }
}

const OUTER_SHELL_SCHEME: &str = "nmp-shell";
const OUTER_SHELL_AUTHORITY: &str = "localhost";
const OUTER_SHELL_PATH: &str = "/trusted-shell.html";
const OUTER_SHELL_CSP: &str = "default-src 'none'; script-src 'self' 'unsafe-inline' nmp-artifact:; style-src 'self' 'unsafe-inline' nmp-artifact:; frame-src 'self' about:; connect-src 'none'; img-src nmp-artifact: data: blob:; media-src nmp-artifact: data: blob:; font-src nmp-artifact: data:; object-src 'none'; base-uri nmp-artifact:; form-action 'none'; worker-src 'none'";
const OUTER_SHELL_DOCUMENT: &[u8] =
    include_bytes!("../../public/trusted-shell/trusted-shell-embedded.html");

fn exact_outer_navigation(url: &tauri::Url) -> bool {
    url.scheme() == OUTER_SHELL_SCHEME
        && url.host_str() == Some(OUTER_SHELL_AUTHORITY)
        && url.port().is_none()
        && url.path() == OUTER_SHELL_PATH
        && url.query().is_none()
        && url.fragment().is_none()
        && url.username().is_empty()
        && url.password().is_none()
}

fn exact_outer_request(method: &tauri::http::Method, uri: &tauri::http::Uri) -> bool {
    method == tauri::http::Method::GET
        && uri.scheme_str() == Some(OUTER_SHELL_SCHEME)
        && uri.authority().map(|value| value.as_str()) == Some(OUTER_SHELL_AUTHORITY)
        && uri.path() == OUTER_SHELL_PATH
        && uri.query().is_none()
}

fn outer_shell_response(
    method: &tauri::http::Method,
    uri: &tauri::http::Uri,
) -> tauri::http::Response<Vec<u8>> {
    let valid = exact_outer_request(method, uri);
    tauri::http::Response::builder()
        .status(if valid {
            tauri::http::StatusCode::OK
        } else {
            tauri::http::StatusCode::NOT_FOUND
        })
        .header("Cache-Control", "no-store, private")
        .header("Content-Security-Policy", OUTER_SHELL_CSP)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("X-Content-Type-Options", "nosniff")
        .body(if valid {
            OUTER_SHELL_DOCUMENT.to_vec()
        } else {
            Vec::new()
        })
        .expect("fixed outer-shell response is valid")
}

fn verify_daemon_compatibility(client: &UnixClient) -> Result<(), String> {
    let response = client
        .request(&Request::Hello { version: VERSION })
        .map_err(|error| format!("UZEL_SHELL_COMPATIBILITY_FAILED {error}"))?;
    verify_daemon_compatibility_response(response)
}

fn verify_daemon_compatibility_response(response: Response) -> Result<(), String> {
    match response {
        Response::Hello { version } if version == VERSION => {
            println!("UZEL_SHELL_COMPATIBLE version={VERSION}");
            Ok(())
        }
        Response::Hello { version } => Err(format!(
            "UZEL_SHELL_COMPATIBILITY_FAILED daemon returned version {version}, expected {VERSION}"
        )),
        _ => Err(
            "UZEL_SHELL_COMPATIBILITY_FAILED daemon returned an unexpected hello response"
                .to_owned(),
        ),
    }
}

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol(OUTER_SHELL_SCHEME, |_context, request| {
            outer_shell_response(request.method(), request.uri())
        })
        .plugin(navigation_policy())
        .setup(|app| {
            let socket = default_socket_path().map_err(|error| error.to_string())?;
            let client = UnixClient::new(socket);
            verify_daemon_compatibility(&client)?;
            app.manage(client);
            app.manage(HostileProbeState::default());
            println!("UZEL_SHELL_READY");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            runtime_status,
            reconcile_runtime,
            select_read_identity,
            runtime_diagnostics,
            start_fixture,
            review_napplet,
            cancel_napplet_review,
            confirm_napplet,
            stop_fixture,
            hostile_probe_enabled,
            webkit_recovery_probe_enabled,
            start_hostile_probe,
            forward_surface_envelope,
            report_shell_accepted,
            report_webkit_recovery,
            hostile_native_probe,
            finish_hostile_probe,
            report_user_mode
        ])
        .run(tauri::generate_context!())
        .expect("Uzel shell failed");
}

fn default_socket_path() -> Result<PathBuf, &'static str> {
    env::var_os("XDG_RUNTIME_DIR")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .map(|path| path.join("uzel/napd.sock"))
        .ok_or("XDG_RUNTIME_DIR is required for the private daemon socket")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_policy_allows_only_trusted_shell_locations() {
        assert!(allowed_navigation(
            &tauri::Url::parse("tauri://localhost/").unwrap()
        ));
        assert!(allowed_navigation(
            &tauri::Url::parse("http://tauri.localhost/").unwrap()
        ));
        assert!(allowed_navigation(
            &tauri::Url::parse("about:srcdoc").unwrap()
        ));
        assert!(allowed_navigation(
            &tauri::Url::parse("nmp-shell://localhost/trusted-shell.html").unwrap()
        ));
        for alias in [
            "nmp-shell://localhost/trusted-shell.html?query",
            "nmp-shell://localhost/trusted-shell.html#fragment",
            "nmp-shell://localhost:80/trusted-shell.html",
            "nmp-shell://user@localhost/trusted-shell.html",
            "nmp-shell://localhost/trusted-shell.html/",
            "nmp-shell://other/trusted-shell.html",
        ] {
            assert!(
                !allowed_navigation(&tauri::Url::parse(alias).unwrap()),
                "{alias}"
            );
        }
        assert!(!allowed_navigation(
            &tauri::Url::parse("https://example.com/").unwrap()
        ));
        assert!(!allowed_navigation(
            &tauri::Url::parse("http://127.0.0.1:43129/probe").unwrap()
        ));
    }

    #[test]
    fn outer_protocol_is_one_exact_get_with_fixed_security_headers() {
        let exact: tauri::http::Uri = "nmp-shell://localhost/trusted-shell.html".parse().unwrap();
        let response = outer_shell_response(&tauri::http::Method::GET, &exact);
        assert_eq!(response.status(), tauri::http::StatusCode::OK);
        assert_eq!(response.body(), OUTER_SHELL_DOCUMENT);
        assert_eq!(
            response.headers()["content-type"],
            "text/html; charset=utf-8"
        );
        assert_eq!(response.headers()["cache-control"], "no-store, private");
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
        assert_eq!(
            response.headers()["content-security-policy"],
            OUTER_SHELL_CSP
        );

        for (method, alias) in [
            (
                tauri::http::Method::POST,
                "nmp-shell://localhost/trusted-shell.html",
            ),
            (
                tauri::http::Method::GET,
                "nmp-shell://localhost/trusted-shell.html?query",
            ),
            (
                tauri::http::Method::GET,
                "nmp-shell://localhost/trusted-shell.html/",
            ),
            (
                tauri::http::Method::GET,
                "nmp-shell://other/trusted-shell.html",
            ),
            (
                tauri::http::Method::GET,
                "nmp-shell://localhost/%74rusted-shell.html",
            ),
        ] {
            let uri: tauri::http::Uri = alias.parse().unwrap();
            let refused = outer_shell_response(&method, &uri);
            assert_eq!(
                refused.status(),
                tauri::http::StatusCode::NOT_FOUND,
                "{alias}"
            );
            assert!(refused.body().is_empty(), "{alias}");
        }
    }

    #[test]
    fn compatible_hello_is_required_before_shell_readiness() {
        assert!(verify_daemon_compatibility_response(Response::Hello { version: VERSION }).is_ok());
        assert!(
            verify_daemon_compatibility_response(Response::Hello {
                version: VERSION.saturating_add(1),
            })
            .unwrap_err()
            .contains("expected")
        );
        assert!(
            verify_daemon_compatibility_response(Response::Shutdown)
                .unwrap_err()
                .contains("unexpected hello")
        );
    }

    #[test]
    fn transfer_cleanup_error_keeps_a_structured_surface_token() {
        let error = ConfirmNappletError::from(ClientError::TransferCleanupFailed {
            surface_token: "surface-needing-retry".to_owned(),
            transfer_error: Box::new(ClientError::InvalidChunk),
            cleanup_error: Box::new(ClientError::UnexpectedResponse),
        });
        let value = serde_json::to_value(error).unwrap();
        assert_eq!(value["kind"], "cleanupRequired");
        assert_eq!(value["surfaceToken"], "surface-needing-retry");
        assert!(
            value["detail"]
                .as_str()
                .unwrap()
                .contains("cleanup also failed")
        );
    }

    #[test]
    fn ambiguous_confirmation_crosses_as_a_typed_retry_state() {
        let error = ConfirmNappletError::from(ClientError::AmbiguousOperation(Box::new(
            ClientError::Protocol(napd_protocol::ProtocolError::Truncated),
        )));
        let value = serde_json::to_value(error).unwrap();
        assert_eq!(value["kind"], "confirmationAmbiguous");
        assert!(value.get("surfaceToken").is_none());
        assert!(value["detail"].as_str().unwrap().contains("peer closed"));
    }

    #[test]
    fn ambiguous_review_crosses_as_a_typed_retry_state() {
        let error = ReviewNappletError::from(ClientError::AmbiguousOperation(Box::new(
            ClientError::Protocol(napd_protocol::ProtocolError::Truncated),
        )));
        let value = serde_json::to_value(error).unwrap();
        assert_eq!(value["kind"], "reviewAmbiguous");
        assert!(value["detail"].as_str().unwrap().contains("peer closed"));
    }

    #[test]
    fn reconciliation_stops_each_snapshot_surface_once() {
        let active = vec![
            "surface-b".to_owned(),
            "surface-a".to_owned(),
            "surface-b".to_owned(),
        ];
        let mut stopped = Vec::new();
        let failures = clean_token_snapshot(&active, |surface_token| {
            stopped.push(surface_token.to_owned());
            Ok(())
        });
        assert_eq!(stopped, ["surface-a", "surface-b"]);
        assert!(failures.is_empty());
    }

    #[test]
    fn post_status_reconciles_lost_stop_replies_and_retains_real_failures() {
        let attempted_failures = BTreeMap::from([
            ("already-stopped".to_owned(), "response lost".to_owned()),
            ("still-live".to_owned(), "daemon unavailable".to_owned()),
        ]);
        let remaining = remaining_cleanup_failures(
            "surface",
            &["newly-observed".to_owned(), "still-live".to_owned()],
            &attempted_failures,
            "daemon still reports the surface after cleanup",
        );
        let values = serde_json::to_value(remaining).unwrap();
        assert_eq!(values[0]["kind"], "surface");
        assert_eq!(values[0]["token"], "newly-observed");
        assert_eq!(
            values[0]["detail"],
            "daemon still reports the surface after cleanup"
        );
        assert_eq!(values[1]["token"], "still-live");
        assert_eq!(values[1]["detail"], "daemon unavailable");
        assert!(
            values
                .as_array()
                .unwrap()
                .iter()
                .all(|entry| entry["token"] != "already-stopped")
        );
    }

    #[test]
    fn review_cleanup_uses_the_same_authoritative_second_snapshot() {
        let remaining = remaining_cleanup_failures(
            "review",
            &["review-still-live".to_owned()],
            &BTreeMap::new(),
            "daemon still reports the review after cancellation",
        );
        let value = serde_json::to_value(remaining).unwrap();
        assert_eq!(value[0]["kind"], "review");
        assert_eq!(value[0]["token"], "review-still-live");
        assert_eq!(
            value[0]["detail"],
            "daemon still reports the review after cancellation"
        );
    }
}
