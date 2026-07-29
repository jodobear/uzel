#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

use napd_protocol::{Request, Response, UnixClient};
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

#[tauri::command]
fn start_fixture(client: tauri::State<'_, UnixClient>) -> Result<SurfaceLaunch, String> {
    let fetched = client.start_fixture().map_err(|error| error.to_string())?;
    let artifact_html = String::from_utf8(fetched.artifact_bytes)
        .map_err(|error| format!("verified artifact is not UTF-8: {error}"))?;
    println!(
        "UZEL_SLICE02_FIXTURE_VERIFIED aggregate={}",
        fetched.surface.aggregate_hash
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
) -> Result<String, String> {
    let response = client
        .request(&Request::ForwardEnvelope {
            surface_token: surface_token.clone(),
            envelope,
        })
        .map_err(|error| error.to_string())?;
    let Response::Envelope { envelope } = response else {
        return Err("daemon returned an unexpected envelope response".to_owned());
    };
    let response_type = serde_json::from_str::<serde_json::Value>(&envelope)
        .ok()
        .and_then(|value| value["type"].as_str().map(str::to_owned))
        .unwrap_or_default();
    match response_type.as_str() {
        "shell.init" => println!("UZEL_SLICE02_HANDSHAKE_OK surface={surface_token}"),
        "identity.getPublicKey.result" => {
            println!("UZEL_SLICE02_ARTIFACT_RESPONDED type={response_type}")
        }
        _ => {}
    }
    Ok(envelope)
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
            app.manage(UnixClient::new(default_socket_path()));
            println!("UZEL_SHELL_READY");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_fixture,
            forward_surface_envelope,
            report_hostile_probe
        ])
        .run(tauri::generate_context!())
        .expect("Uzel shell failed");
}

fn default_socket_path() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join("uzel/napd.sock")
}
