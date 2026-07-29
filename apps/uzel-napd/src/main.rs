#![forbid(unsafe_code)]

use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
};

use napd::{DaemonServer, LinuxRunner};

#[derive(Debug)]
struct Options {
    check: bool,
    live: bool,
    socket: PathBuf,
    runtime_root: PathBuf,
    indexer_relays: Vec<String>,
    app_relays: Vec<String>,
    fallback_relays: Vec<String>,
    allowed_local_relay_hosts: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = parse_options(env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    if options.check {
        println!("UZEL_NAPD_READY role={}", napd::PROCESS_ROLE);
        return Ok(());
    }

    let runner = if options.live {
        LinuxRunner::open_live(
            &options.runtime_root,
            options.indexer_relays,
            options.app_relays,
            options.fallback_relays,
            options.allowed_local_relay_hosts,
        )?
    } else {
        LinuxRunner::open(&options.runtime_root)?
    };
    let server = DaemonServer::bind(&options.socket, runner)?;
    println!(
        "UZEL_NAPD_READY role={} mode={} socket={}",
        napd::PROCESS_ROLE,
        if options.live { "live" } else { "fixture" },
        options.socket.display()
    );
    io::stdout().flush()?;
    server.serve()?;
    Ok(())
}

fn parse_options(arguments: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut check = false;
    let mut live = false;
    let mut socket = None;
    let mut runtime_root = None;
    let mut indexer_relays = Vec::new();
    let mut app_relays = Vec::new();
    let mut fallback_relays = Vec::new();
    let mut allowed_local_relay_hosts = Vec::new();
    let mut arguments = arguments.into_iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => check = true,
            "--live" => live = true,
            "--socket" => socket = Some(next_path(&mut arguments, "--socket")?),
            "--runtime-root" => runtime_root = Some(next_path(&mut arguments, "--runtime-root")?),
            "--indexer-relay" => {
                indexer_relays.push(next_value(&mut arguments, "--indexer-relay")?)
            }
            "--app-relay" => app_relays.push(next_value(&mut arguments, "--app-relay")?),
            "--fallback-relay" => {
                fallback_relays.push(next_value(&mut arguments, "--fallback-relay")?)
            }
            "--allow-local-relay-host" => allowed_local_relay_hosts
                .push(next_value(&mut arguments, "--allow-local-relay-host")?),
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }
    if !live
        && (!indexer_relays.is_empty()
            || !app_relays.is_empty()
            || !fallback_relays.is_empty()
            || !allowed_local_relay_hosts.is_empty())
    {
        return Err("relay configuration requires --live".to_owned());
    }
    Ok(Options {
        check,
        live,
        socket: socket.unwrap_or_else(default_socket_path),
        runtime_root: runtime_root.unwrap_or_else(default_runtime_root),
        indexer_relays,
        app_relays,
        fallback_relays,
        allowed_local_relay_hosts,
    })
}

fn next_value(
    arguments: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<String, String> {
    arguments
        .next()
        .filter(|value| !value.is_empty() && !value.chars().any(char::is_control))
        .ok_or_else(|| format!("{option} requires a control-free value"))
}

fn next_path(
    arguments: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<PathBuf, String> {
    next_value(arguments, option).map(PathBuf::from)
}

fn default_socket_path() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join("uzel/napd.sock")
}

fn default_runtime_root() -> PathBuf {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .unwrap_or_else(env::temp_dir)
        .join("uzel")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_relays_require_explicit_live_mode() {
        assert!(
            parse_options(["--app-relay".to_owned(), "wss://relay.example".to_owned()]).is_err()
        );
        let parsed = parse_options([
            "--live".to_owned(),
            "--app-relay".to_owned(),
            "wss://relay.example".to_owned(),
        ])
        .unwrap();
        assert!(parsed.live);
        assert_eq!(parsed.app_relays, ["wss://relay.example"]);
    }
}
