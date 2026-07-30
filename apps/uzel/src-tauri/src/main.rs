#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

use napd_protocol::{
    ClientError, Diagnostics, FetchedSurface, NappletReview, Request, Response, RoutedEnvelope,
    UnixClient,
};
use serde::Serialize;
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
    active_identity: Option<String>,
}

#[tauri::command]
fn runtime_status(client: tauri::State<'_, UnixClient>) -> Result<RuntimeStatus, String> {
    match client
        .request(&Request::Status)
        .map_err(|error| error.to_string())?
    {
        Response::Status {
            mode,
            active_surfaces,
            active_identity,
            ..
        } => Ok(RuntimeStatus {
            mode,
            active_surfaces,
            active_identity,
        }),
        _ => Err("daemon returned an unexpected status response".to_owned()),
    }
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
) -> Result<NappletReview, String> {
    client
        .review_napplet(&coordinate)
        .map_err(|error| error.to_string())
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
        domains: fetched.surface.domains,
        unavailable_domains: fetched.surface.unavailable_domains,
    })
}

#[tauri::command]
fn hostile_probe_enabled() -> bool {
    env::var("UZEL_RUN_HOSTILE_PROBE").as_deref() == Ok("1")
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
fn stop_fixture(client: tauri::State<'_, UnixClient>, surface_token: String) -> Result<(), String> {
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

fn main() {
    tauri::Builder::default()
        .plugin(navigation_policy())
        .setup(|app| {
            let socket = default_socket_path().map_err(|error| error.to_string())?;
            app.manage(UnixClient::new(socket));
            app.manage(HostileProbeState::default());
            println!("UZEL_SHELL_READY");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            runtime_status,
            select_read_identity,
            runtime_diagnostics,
            start_fixture,
            review_napplet,
            cancel_napplet_review,
            confirm_napplet,
            stop_fixture,
            hostile_probe_enabled,
            start_hostile_probe,
            forward_surface_envelope,
            report_shell_accepted,
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
        assert!(!allowed_navigation(
            &tauri::Url::parse("https://example.com/").unwrap()
        ));
        assert!(!allowed_navigation(
            &tauri::Url::parse("http://127.0.0.1:43129/probe").unwrap()
        ));
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
}
