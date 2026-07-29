#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

use napd_protocol::{Diagnostics, Request, Response, RoutedEnvelope, UnixClient};
use serde::{Deserialize, Serialize};
use tauri::Manager;

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
    let artifact_html = String::from_utf8(fetched.artifact_bytes)
        .map_err(|error| format!("verified artifact is not UTF-8: {error}"))?;
    println!(
        "UZEL_FIXTURE_VERIFIED fixture={} aggregate={}",
        fixture, fetched.surface.aggregate_hash
    );
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostileProbe {
    tauri_internals: bool,
    tauri_global: bool,
    wry_ipc: bool,
    parent_readable: bool,
    raw_webkit_transport: bool,
}

#[tauri::command]
fn report_hostile_probe(report: HostileProbe) -> Result<(), String> {
    if report.tauri_internals || report.tauri_global || report.wry_ipc || report.parent_readable {
        return Err("sandboxed child retained trusted host authority".to_owned());
    }
    println!(
        "UZEL_SLICE02_ISOLATION_OK raw_webkit_transport={}",
        report.raw_webkit_transport
    );
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let socket = default_socket_path().map_err(|error| error.to_string())?;
            app.manage(UnixClient::new(socket));
            println!("UZEL_SHELL_READY");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            runtime_status,
            select_read_identity,
            runtime_diagnostics,
            start_fixture,
            forward_surface_envelope,
            report_shell_accepted,
            report_hostile_probe
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
