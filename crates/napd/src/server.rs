use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    io::{self, Write},
    os::unix::{
        fs::{FileTypeExt, MetadataExt, PermissionsExt},
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
    time::Duration,
};

use napd_protocol::{
    ASSET_CHUNK_BYTES, MAX_ASSET_BYTES, ProtocolError, Request, Response, VERSION,
    encode_asset_chunk, read_frame, write_frame,
};

use crate::{LinuxRunner, RunnerError, SurfaceLaunch};

const STREAM_TIMEOUT: Duration = Duration::from_secs(2);
const MAXIMUM_REPLAY_OPERATIONS: usize = 64;
const MAXIMUM_OPERATION_ID_BYTES: usize = 128;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("daemon socket parent could not be prepared: {0}")]
    SocketParent(io::Error),
    #[error("refusing unsafe stale daemon socket at {0}")]
    UnsafeStaleSocket(PathBuf),
    #[error("refusing non-directory or symlink daemon socket parent at {0}")]
    UnsafeSocketParent(PathBuf),
    #[error("refusing insecure daemon socket parent permissions at {0}")]
    InsecureSocketParent(PathBuf),
    #[error("another daemon is already listening at {0}")]
    ActiveSocket(PathBuf),
    #[error("daemon socket activity probe failed: {0}")]
    SocketProbe(io::Error),
    #[error("daemon socket could not bind: {0}")]
    Bind(io::Error),
    #[error("daemon socket permissions could not be set: {0}")]
    Permissions(io::Error),
    #[error("daemon socket accept failed: {0}")]
    Accept(io::Error),
}

#[derive(Debug)]
struct AssetTransfer {
    id: String,
    bytes: Vec<u8>,
    next_offset: usize,
}

#[derive(Debug)]
struct ReplayEntry {
    request: Request,
    response: Response,
}

#[derive(Debug)]
struct InvalidOperationId;

#[derive(Debug, Default)]
struct ReplayCache {
    entries: BTreeMap<String, ReplayEntry>,
    order: VecDeque<String>,
}

impl ReplayCache {
    fn lookup(&self, key: &str, request: &Request) -> Option<Response> {
        self.entries.get(key).map(|entry| {
            if entry.request == *request {
                entry.response.clone()
            } else {
                Response::error(
                    "operation_id_reused",
                    "operation id is already bound to a different request",
                )
            }
        })
    }

    fn remember(&mut self, key: String, request: Request, response: Response) {
        if self.entries.contains_key(&key) {
            return;
        }
        while self.order.len() >= MAXIMUM_REPLAY_OPERATIONS {
            if let Some(expired) = self.order.pop_front() {
                self.entries.remove(&expired);
            }
        }
        self.order.push_back(key.clone());
        self.entries.insert(key, ReplayEntry { request, response });
    }
}

#[derive(Debug)]
pub struct DaemonState {
    runner: LinuxRunner,
    transfers: BTreeMap<String, AssetTransfer>,
    replay: ReplayCache,
}

impl DaemonState {
    pub fn new(runner: LinuxRunner) -> Self {
        Self {
            runner,
            transfers: BTreeMap::new(),
            replay: ReplayCache::default(),
        }
    }

    pub fn handle(&mut self, request: Request) -> (Response, bool) {
        let replay_key = match replay_key(&request) {
            Ok(replay_key) => replay_key,
            Err(InvalidOperationId) => {
                return (
                    Response::error(
                        "invalid_operation_id",
                        "operation id must be 1-128 ASCII letters, digits, dots, underscores, or hyphens",
                    ),
                    false,
                );
            }
        };
        if let Some(key) = replay_key.as_deref()
            && let Some(response) = self.replay.lookup(key, &request)
        {
            return (response, false);
        }
        let replay_request = replay_key.as_ref().map(|_| request.clone());
        let response = match request {
            Request::Hello { version } if version == VERSION => Response::Hello { version },
            Request::Hello { version } => Response::error(
                "version_mismatch",
                format!("daemon protocol is {VERSION}, client requested {version}"),
            ),
            Request::Status => match self.runner.get_read_identity() {
                Ok(active_identity) => Response::Status {
                    version: VERSION,
                    mode: self.runner.mode().as_str().to_owned(),
                    active_surfaces: self.runner.active_surfaces(),
                    active_identity,
                },
                Err(error) => runner_error(error),
            },
            Request::StartFixture { fixture } => {
                let result = self.runner.start_named_fixture(&fixture);
                self.stage_surface(result)
            }
            Request::StartHostileProbe { sentinel_url } => {
                let result = self.runner.start_hostile_probe(&sentinel_url);
                self.stage_surface(result)
            }
            Request::ReviewNapplet {
                operation_id: _,
                coordinate,
            } => match self.runner.review_napplet(coordinate) {
                Ok(review) => Response::NappletReview { review },
                Err(error) => runner_error(error),
            },
            Request::CancelNappletReview { token } => {
                match self.runner.cancel_napplet_review(&token) {
                    Ok(()) => Response::ReviewCancelled,
                    Err(error) => runner_error(error),
                }
            }
            Request::ConfirmNapplet {
                operation_id: _,
                token,
                expected_author,
                expected_d_tag,
                expected_aggregate_hash,
                granted_domains,
            } => {
                let result = self.runner.confirm_napplet(
                    token,
                    expected_author,
                    expected_d_tag,
                    expected_aggregate_hash,
                    granted_domains,
                );
                self.stage_surface(result)
            }
            Request::StopFixture { surface_token } => {
                match self.runner.stop_fixture(&surface_token) {
                    Ok(()) => {
                        self.transfers
                            .remove(&format!("fixture-index-{surface_token}"));
                        Response::Stopped
                    }
                    Err(error) => runner_error(error),
                }
            }
            Request::AssetChunk {
                transfer_id,
                offset,
            } => self.asset_chunk(&transfer_id, offset),
            Request::ForwardEnvelope {
                surface_token,
                envelope,
            } => match self
                .runner
                .forward_from_surface(&surface_token, envelope.as_bytes())
            {
                Ok(delivery) => Response::Envelope {
                    surface_token: delivery.surface_token,
                    envelope: delivery.envelope,
                },
                Err(error) => runner_error(error),
            },
            Request::SetReadIdentity { public_identity } => {
                match self.runner.set_read_identity(public_identity) {
                    Ok(active_public_key) => Response::Identity {
                        active_public_key: Some(active_public_key),
                    },
                    Err(error) => runner_error(error),
                }
            }
            Request::GetReadIdentity => match self.runner.get_read_identity() {
                Ok(active_public_key) => Response::Identity { active_public_key },
                Err(error) => runner_error(error),
            },
            Request::Diagnostics => match self.runner.diagnostics() {
                Ok(diagnostics) => Response::Diagnostics { diagnostics },
                Err(error) => runner_error(error),
            },
            Request::Shutdown => Response::Shutdown,
        };
        if let (Some(key), Some(request)) = (replay_key, replay_request) {
            self.replay.remember(key, request, response.clone());
        }
        let shutdown = matches!(response, Response::Shutdown);
        (response, shutdown)
    }

    fn asset_chunk(&mut self, transfer_id: &str, offset: u64) -> Response {
        let Some(transfer) = self.transfers.get_mut(transfer_id) else {
            return Response::error("unknown_transfer", "no verified asset transfer is active");
        };
        let Ok(offset) = usize::try_from(offset) else {
            return Response::error("invalid_offset", "asset offset does not fit this host");
        };
        if offset != transfer.next_offset {
            return Response::error(
                "out_of_order",
                format!(
                    "expected asset offset {}, received {offset}",
                    transfer.next_offset
                ),
            );
        }
        let end = offset
            .saturating_add(ASSET_CHUNK_BYTES)
            .min(transfer.bytes.len());
        let bytes_base64 = encode_asset_chunk(&transfer.bytes[offset..end]);
        transfer.next_offset = end;
        Response::AssetChunk {
            transfer_id: transfer.id.clone(),
            offset: offset as u64,
            next_offset: end as u64,
            total_bytes: transfer.bytes.len() as u64,
            bytes_base64,
            done: end == transfer.bytes.len(),
        }
    }

    fn stage_surface(&mut self, result: Result<SurfaceLaunch, RunnerError>) -> Response {
        match result {
            Ok(launch) if launch.artifact_html.len() <= MAX_ASSET_BYTES => {
                let total_bytes = launch.artifact_html.len() as u64;
                let surface = launch.metadata();
                let transfer_id = format!("fixture-index-{}", surface.surface_token);
                self.transfers.insert(
                    transfer_id.clone(),
                    AssetTransfer {
                        id: transfer_id.clone(),
                        bytes: launch.artifact_html.into_bytes(),
                        next_offset: 0,
                    },
                );
                Response::Surface {
                    surface,
                    transfer_id,
                    total_bytes,
                }
            }
            Ok(_) => Response::error(
                "asset_too_large",
                format!("verified document exceeds {MAX_ASSET_BYTES} bytes"),
            ),
            Err(error) => runner_error(error),
        }
    }
}

fn replay_key(request: &Request) -> Result<Option<String>, InvalidOperationId> {
    let (kind, operation_id) = match request {
        Request::ReviewNapplet { operation_id, .. } => ("review", operation_id),
        Request::ConfirmNapplet { operation_id, .. } => ("confirm", operation_id),
        _ => return Ok(None),
    };
    if operation_id.is_empty()
        || operation_id.len() > MAXIMUM_OPERATION_ID_BYTES
        || !operation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(InvalidOperationId);
    }
    Ok(Some(format!("{kind}:{operation_id}")))
}

fn runner_error(error: RunnerError) -> Response {
    Response::error("runtime_refused", bounded_detail(error.to_string()))
}

fn bounded_detail(mut detail: String) -> String {
    const MAXIMUM_DETAIL_BYTES: usize = 1_024;
    if detail.len() > MAXIMUM_DETAIL_BYTES {
        let mut boundary = MAXIMUM_DETAIL_BYTES;
        while !detail.is_char_boundary(boundary) {
            boundary -= 1;
        }
        detail.truncate(boundary);
    }
    detail
}

#[derive(Debug)]
pub struct DaemonServer {
    listener: UnixListener,
    socket_path: PathBuf,
    socket_identity: SocketIdentity,
    state: DaemonState,
}

#[derive(Debug)]
struct SocketIdentity {
    device: u64,
    inode: u64,
}

impl DaemonServer {
    pub fn bind(socket_path: impl AsRef<Path>, runner: LinuxRunner) -> Result<Self, ServerError> {
        let socket_path = socket_path.as_ref().to_path_buf();
        prepare_socket_parent(&socket_path)?;
        remove_owned_stale_socket(&socket_path)?;
        let listener = UnixListener::bind(&socket_path).map_err(ServerError::Bind)?;
        fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o600))
            .map_err(ServerError::Permissions)?;
        let metadata = fs::symlink_metadata(&socket_path).map_err(ServerError::Bind)?;
        Ok(Self {
            listener,
            socket_path,
            socket_identity: SocketIdentity {
                device: metadata.dev(),
                inode: metadata.ino(),
            },
            state: DaemonState::new(runner),
        })
    }

    pub fn serve(mut self) -> Result<(), ServerError> {
        loop {
            let (mut stream, _) = self.listener.accept().map_err(ServerError::Accept)?;
            stream
                .set_read_timeout(Some(STREAM_TIMEOUT))
                .map_err(ServerError::SocketProbe)?;
            stream
                .set_write_timeout(Some(STREAM_TIMEOUT))
                .map_err(ServerError::SocketProbe)?;
            let shutdown = handle_stream(&mut self.state, &mut stream);
            if shutdown {
                return Ok(());
            }
        }
    }
}

impl Drop for DaemonServer {
    fn drop(&mut self) {
        let Ok(metadata) = fs::symlink_metadata(&self.socket_path) else {
            return;
        };
        if metadata.file_type().is_socket()
            && metadata.dev() == self.socket_identity.device
            && metadata.ino() == self.socket_identity.inode
        {
            let _ = fs::remove_file(&self.socket_path);
        }
    }
}

fn handle_stream(state: &mut DaemonState, stream: &mut UnixStream) -> bool {
    let request = match read_frame::<Request>(stream) {
        Ok(request) => request,
        Err(error) => {
            let _ = write_frame(
                stream,
                &Response::error("invalid_frame", bounded_detail(error.to_string())),
            );
            return false;
        }
    };
    let (response, shutdown) = state.handle(request);
    if !write_response(stream, &response) {
        return false;
    }
    shutdown
}

fn write_response(stream: &mut UnixStream, response: &Response) -> bool {
    let mut frame = Vec::new();
    match write_frame(&mut frame, response) {
        Ok(()) => stream
            .write_all(&frame)
            .and_then(|()| stream.flush())
            .is_ok(),
        Err(ProtocolError::FrameTooLarge { .. }) => write_frame(
            stream,
            &Response::error(
                "response_too_large",
                "runtime response exceeds the private control-frame limit",
            ),
        )
        .is_ok(),
        Err(_) => false,
    }
}

fn prepare_socket_parent(socket_path: &Path) -> Result<(), ServerError> {
    let parent = socket_path
        .parent()
        .ok_or_else(|| ServerError::UnsafeStaleSocket(socket_path.to_path_buf()))?;
    let created = match fs::symlink_metadata(parent) {
        Ok(_) => false,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(parent).map_err(ServerError::SocketParent)?;
            true
        }
        Err(error) => return Err(ServerError::SocketParent(error)),
    };
    let metadata = fs::symlink_metadata(parent).map_err(ServerError::SocketParent)?;
    if !metadata.file_type().is_dir() {
        return Err(ServerError::UnsafeSocketParent(parent.to_path_buf()));
    }
    if created {
        fs::set_permissions(parent, fs::Permissions::from_mode(0o700))
            .map_err(ServerError::Permissions)?;
    } else if metadata.permissions().mode() & 0o077 != 0 {
        return Err(ServerError::InsecureSocketParent(parent.to_path_buf()));
    }
    Ok(())
}

fn remove_owned_stale_socket(socket_path: &Path) -> Result<(), ServerError> {
    let metadata = match fs::symlink_metadata(socket_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(ServerError::Bind(error)),
    };
    let parent = socket_path
        .parent()
        .ok_or_else(|| ServerError::UnsafeStaleSocket(socket_path.to_path_buf()))?;
    let parent_metadata = fs::metadata(parent).map_err(ServerError::SocketParent)?;
    if !metadata.file_type().is_socket() || metadata.uid() != parent_metadata.uid() {
        return Err(ServerError::UnsafeStaleSocket(socket_path.to_path_buf()));
    }
    match UnixStream::connect(socket_path) {
        Ok(_) => return Err(ServerError::ActiveSocket(socket_path.to_path_buf())),
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::ConnectionRefused | io::ErrorKind::NotFound
            ) => {}
        Err(error) => return Err(ServerError::SocketProbe(error)),
    }
    let current = fs::symlink_metadata(socket_path).map_err(ServerError::Bind)?;
    if !current.file_type().is_socket()
        || current.dev() != metadata.dev()
        || current.ino() != metadata.ino()
    {
        return Err(ServerError::UnsafeStaleSocket(socket_path.to_path_buf()));
    }
    fs::remove_file(socket_path).map_err(ServerError::Bind)
}

#[cfg(test)]
mod tests {
    use std::os::unix::{fs::symlink, net::UnixStream};

    use napd_protocol::{Request, Response, UnixClient, read_frame, write_frame};
    use tempfile::TempDir;

    use super::*;

    const FIXTURE_INDEX: &[u8] = include_bytes!("../../../fixtures/good-morning/index.html");

    fn exchange(socket: &Path, request: &Request) -> Response {
        let mut stream = UnixStream::connect(socket).unwrap();
        write_frame(&mut stream, request).unwrap();
        read_frame(&mut stream).unwrap()
    }

    #[test]
    fn daemon_serves_ordered_verified_asset_and_shuts_down() {
        let temp = TempDir::new().unwrap();
        let socket = temp.path().join("run/uzel.sock");
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let server = DaemonServer::bind(&socket, runner).unwrap();
        let thread = std::thread::spawn(move || server.serve().unwrap());

        assert_eq!(
            exchange(&socket, &Request::Hello { version: VERSION }),
            Response::Hello { version: VERSION }
        );
        let (transfer_id, total_bytes) = match exchange(
            &socket,
            &Request::StartFixture {
                fixture: "good-morning".to_owned(),
            },
        ) {
            Response::Surface {
                transfer_id,
                total_bytes,
                ..
            } => (transfer_id, total_bytes),
            response => panic!("unexpected start response: {response:?}"),
        };
        assert!(matches!(
            exchange(
                &socket,
                &Request::AssetChunk {
                    transfer_id: transfer_id.clone(),
                    offset: 1,
                }
            ),
            Response::Error { ref code, .. } if code == "out_of_order"
        ));
        assert_eq!(total_bytes, FIXTURE_INDEX.len() as u64);
        let fetched = UnixClient::new(&socket).start_fixture().unwrap();
        assert_eq!(fetched.artifact_bytes, FIXTURE_INDEX);
        assert_eq!(exchange(&socket, &Request::Shutdown), Response::Shutdown);
        thread.join().unwrap();
        assert!(!socket.exists());
    }

    #[test]
    fn server_drop_does_not_remove_a_replacement_socket() {
        let temp = TempDir::new().unwrap();
        let socket = temp.path().join("run/uzel.sock");
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let server = DaemonServer::bind(&socket, runner).unwrap();
        fs::remove_file(&socket).unwrap();
        let replacement = UnixListener::bind(&socket).unwrap();
        drop(server);
        assert!(socket.exists());
        drop(replacement);
        fs::remove_file(&socket).unwrap();
    }

    #[test]
    fn socket_parent_symlink_is_refused() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("target");
        fs::create_dir(&target).unwrap();
        let parent = temp.path().join("run");
        symlink(&target, &parent).unwrap();
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        assert!(matches!(
            DaemonServer::bind(parent.join("uzel.sock"), runner),
            Err(ServerError::UnsafeSocketParent(_))
        ));
    }

    #[test]
    fn active_daemon_socket_is_not_unlinked() {
        let temp = TempDir::new().unwrap();
        let socket = temp.path().join("run/uzel.sock");
        let first_runner = LinuxRunner::open(temp.path().join("state-one")).unwrap();
        let first = DaemonServer::bind(&socket, first_runner).unwrap();
        let second_runner = LinuxRunner::open(temp.path().join("state-two")).unwrap();
        assert!(matches!(
            DaemonServer::bind(&socket, second_runner),
            Err(ServerError::ActiveSocket(_))
        ));
        assert!(socket.exists());
        drop(first);
    }

    #[test]
    fn existing_shared_socket_parent_is_not_chmodded() {
        let temp = TempDir::new().unwrap();
        fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();
        let socket = temp.path().join("uzel.sock");
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        assert!(matches!(
            DaemonServer::bind(&socket, runner),
            Err(ServerError::InsecureSocketParent(_))
        ));
        assert_eq!(
            fs::metadata(temp.path()).unwrap().permissions().mode() & 0o777,
            0o755
        );
    }

    #[test]
    fn incomplete_client_times_out_without_blocking_the_next_request() {
        let temp = TempDir::new().unwrap();
        let socket = temp.path().join("run/uzel.sock");
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let server = DaemonServer::bind(&socket, runner).unwrap();
        let thread = std::thread::spawn(move || server.serve().unwrap());
        let mut stalled = UnixStream::connect(&socket).unwrap();
        stalled
            .set_read_timeout(Some(STREAM_TIMEOUT + Duration::from_secs(1)))
            .unwrap();
        assert!(matches!(
            read_frame::<Response>(&mut stalled).unwrap(),
            Response::Error { ref code, .. } if code == "invalid_frame"
        ));
        assert_eq!(
            exchange(&socket, &Request::Hello { version: VERSION }),
            Response::Hello { version: VERSION }
        );
        assert_eq!(exchange(&socket, &Request::Shutdown), Response::Shutdown);
        thread.join().unwrap();
    }

    #[test]
    fn daemon_routes_inc_delivery_to_the_other_exact_surface() {
        let temp = TempDir::new().unwrap();
        let socket = temp.path().join("run/uzel.sock");
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let server = DaemonServer::bind(&socket, runner).unwrap();
        let thread = std::thread::spawn(move || server.serve().unwrap());
        let client = UnixClient::new(&socket);
        let follow = client.start_named_fixture("follow-list").unwrap();
        let profile = client.start_named_fixture("profile-card").unwrap();

        for surface in [&follow.surface, &profile.surface] {
            assert!(matches!(
                client
                    .request(&Request::ForwardEnvelope {
                        surface_token: surface.surface_token.clone(),
                        envelope: r#"{"type":"shell.ready"}"#.to_owned(),
                    })
                    .unwrap(),
                Response::Envelope { surface_token, ref envelope }
                    if surface_token == surface.surface_token
                        && envelope.contains(r#""type":"shell.init""#)
            ));
        }

        assert!(matches!(
            client
                .request(&Request::ForwardEnvelope {
                    surface_token: profile.surface.surface_token.clone(),
                    envelope: r#"{"type":"inc.subscribe","id":"profile-open-sub","topic":"napplet:profile/open"}"#.to_owned(),
                })
                .unwrap(),
            Response::Envelope { surface_token, ref envelope }
                if surface_token == profile.surface.surface_token
                    && envelope.contains(r#""type":"inc.subscribe.result""#)
        ));

        let delivery = client
            .request(&Request::ForwardEnvelope {
                surface_token: follow.surface.surface_token,
                envelope: r#"{"type":"inc.emit","topic":"napplet:profile/open","payload":{"version":1,"pubkey":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#.to_owned(),
            })
            .unwrap();
        assert!(matches!(
            delivery,
            Response::Envelope { surface_token, ref envelope }
                if surface_token == profile.surface.surface_token
                    && envelope.contains(r#""sender":"follow-list""#)
        ));
        assert_eq!(
            client.request(&Request::Shutdown).unwrap(),
            Response::Shutdown
        );
        thread.join().unwrap();
    }

    #[test]
    fn oversized_runtime_response_becomes_a_typed_error_frame() {
        let (mut server, mut client) = UnixStream::pair().unwrap();
        assert!(write_response(
            &mut server,
            &Response::Envelope {
                surface_token: "oversized".to_owned(),
                envelope: "x".repeat(napd_protocol::MAX_FRAME_BYTES),
            }
        ));
        assert_eq!(
            read_frame::<Response>(&mut client).unwrap(),
            Response::error(
                "response_too_large",
                "runtime response exceeds the private control-frame limit"
            )
        );
    }

    #[test]
    fn replay_cache_binds_operation_id_to_the_exact_confirm_request() {
        let temp = TempDir::new().unwrap();
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let mut state = DaemonState::new(runner);
        let request = Request::ConfirmNapplet {
            operation_id: "confirm-1".to_owned(),
            token: "missing-review".to_owned(),
            expected_author: "a".repeat(64),
            expected_d_tag: "test".to_owned(),
            expected_aggregate_hash: "b".repeat(64),
            granted_domains: Vec::new(),
        };
        let first = state.handle(request.clone()).0;
        assert!(matches!(
            first,
            Response::Error { ref code, .. } if code == "runtime_refused"
        ));
        assert_eq!(state.handle(request.clone()).0, first);

        let Request::ConfirmNapplet {
            operation_id,
            expected_author,
            expected_d_tag,
            expected_aggregate_hash,
            granted_domains,
            ..
        } = request
        else {
            unreachable!()
        };
        assert!(matches!(
            state
                .handle(Request::ConfirmNapplet {
                    operation_id,
                    token: "different-review".to_owned(),
                    expected_author,
                    expected_d_tag,
                    expected_aggregate_hash,
                    granted_domains,
                })
                .0,
            Response::Error { ref code, .. } if code == "operation_id_reused"
        ));
    }

    #[test]
    fn replay_cache_is_bounded_and_operation_ids_are_validated() {
        let mut cache = ReplayCache::default();
        for index in 0..=MAXIMUM_REPLAY_OPERATIONS {
            let request = Request::ReviewNapplet {
                operation_id: format!("review-{index}"),
                coordinate: format!("coordinate-{index}"),
            };
            cache.remember(
                format!("review:review-{index}"),
                request,
                Response::error("test", "cached"),
            );
        }
        assert_eq!(cache.entries.len(), MAXIMUM_REPLAY_OPERATIONS);
        assert!(!cache.entries.contains_key("review:review-0"));
        assert!(
            cache
                .entries
                .contains_key(&format!("review:review-{MAXIMUM_REPLAY_OPERATIONS}"))
        );

        let temp = TempDir::new().unwrap();
        let runner = LinuxRunner::open(temp.path().join("state")).unwrap();
        let mut state = DaemonState::new(runner);
        assert!(matches!(
            state
                .handle(Request::ReviewNapplet {
                    operation_id: "bad id".to_owned(),
                    coordinate: "unused".to_owned(),
                })
                .0,
            Response::Error { ref code, .. } if code == "invalid_operation_id"
        ));
    }
}
