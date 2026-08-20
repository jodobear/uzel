#![forbid(unsafe_code)]

use std::{
    env,
    error::Error,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use napd::{DaemonServer, LinuxRunner};

#[derive(Debug)]
struct Options {
    check: bool,
    live: bool,
    ready_fd: Option<u32>,
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
    apply_test_pre_bind_delay()?;
    let server = DaemonServer::bind(&options.socket, runner)?;
    report_launcher_ready(options.ready_fd, server.socket_identity())?;
    println!("UZEL_NAPD_READY role={}", napd::PROCESS_ROLE);
    println!(
        "UZEL_NAPD_ENDPOINT mode={} socket={}",
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
    let mut ready_fd = None;
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
            "--ready-fd" => ready_fd = Some(next_fd(&mut arguments)?),
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
        return Err("live daemon configuration requires --live".to_owned());
    }
    Ok(Options {
        check,
        live,
        ready_fd,
        socket: match socket {
            Some(path) => path,
            None if check => PathBuf::new(),
            None => default_socket_path()?,
        },
        runtime_root: match runtime_root {
            Some(path) => path,
            None if check => PathBuf::new(),
            None => default_runtime_root()?,
        },
        indexer_relays,
        app_relays,
        fallback_relays,
        allowed_local_relay_hosts,
    })
}

fn next_fd(arguments: &mut impl Iterator<Item = String>) -> Result<u32, String> {
    next_value(arguments, "--ready-fd")?
        .parse::<u32>()
        .ok()
        .filter(|fd| (3..=1_024).contains(fd))
        .ok_or_else(|| "--ready-fd requires an integer from 3 through 1024".to_owned())
}

fn apply_test_pre_bind_delay() -> Result<(), io::Error> {
    let Some(value) = env::var_os("UZEL_NAPD_TEST_PRE_BIND_DELAY_MS") else {
        return Ok(());
    };
    let milliseconds = value
        .to_str()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| (1..=10_000).contains(value))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "UZEL_NAPD_TEST_PRE_BIND_DELAY_MS must be an integer from 1 through 10000",
            )
        })?;
    thread::sleep(Duration::from_millis(milliseconds));
    Ok(())
}

fn report_launcher_ready(
    ready_fd: Option<u32>,
    socket_identity: (u64, u64),
) -> Result<(), io::Error> {
    let Some(ready_fd) = ready_fd else {
        return Ok(());
    };
    let mut channel = OpenOptions::new()
        .write(true)
        .open(format!("/proc/self/fd/{ready_fd}"))?;
    writeln!(
        channel,
        "UZEL_NAPD_BOUND {}:{}",
        socket_identity.0, socket_identity.1
    )?;
    channel.flush()
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

fn default_socket_path() -> Result<PathBuf, String> {
    env::var_os("XDG_RUNTIME_DIR")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .map(|path| path.join("uzel/napd.sock"))
        .ok_or_else(|| "XDG_RUNTIME_DIR is required unless --socket is supplied".to_owned())
}

fn default_runtime_root() -> Result<PathBuf, String> {
    env::var_os("XDG_DATA_HOME")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .map(|path| path.join("uzel"))
        .ok_or_else(|| {
            "XDG_DATA_HOME or HOME is required unless --runtime-root is supplied".to_owned()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_configuration_requires_explicit_live_mode() {
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
        assert!(parse_options(["--live".to_owned()]).is_ok());
        assert_eq!(
            parse_options(["--live".to_owned(), "--ready-fd".to_owned(), "8".to_owned(),])
                .unwrap()
                .ready_fd,
            Some(8)
        );
        assert!(parse_options(["--ready-fd".to_owned(), "2".to_owned()]).is_err());
        assert!(
            parse_options([
                "--live".to_owned(),
                "--resource-blossom-server".to_owned(),
                "https://blossom.example".to_owned(),
            ])
            .is_err()
        );
    }
}
