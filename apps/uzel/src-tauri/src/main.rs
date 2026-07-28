#![forbid(unsafe_code)]

use std::sync::Mutex;

use napd::{LinuxRunner, SurfaceLaunch};
use serde::Deserialize;
use tauri::Manager;

#[tauri::command]
fn start_fixture(runner: tauri::State<'_, Mutex<LinuxRunner>>) -> Result<SurfaceLaunch, String> {
    let launch = runner
        .lock()
        .map_err(|_| "runner lock poisoned".to_owned())?
        .start_fixture()
        .map_err(|error| error.to_string())?;
    println!(
        "UZEL_SLICE02_FIXTURE_VERIFIED aggregate={}",
        launch.aggregate_hash
    );
    Ok(launch)
}

#[tauri::command]
fn forward_surface_envelope(
    runner: tauri::State<'_, Mutex<LinuxRunner>>,
    surface_token: String,
    envelope: String,
) -> Result<String, String> {
    let response = runner
        .lock()
        .map_err(|_| "runner lock poisoned".to_owned())?
        .forward_from_surface(&surface_token, envelope.as_bytes())
        .map_err(|error| error.to_string())?;
    let response_type = serde_json::from_str::<serde_json::Value>(&response)
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
    Ok(response)
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
            let runtime_root = app
                .path()
                .app_local_data_dir()
                .map_err(|error| format!("app data directory unavailable: {error}"))?
                .join("slice-02-runtime");
            let runner = LinuxRunner::open(runtime_root)
                .map_err(|error| format!("Linux runner failed: {error}"))?;
            app.manage(Mutex::new(runner));
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
