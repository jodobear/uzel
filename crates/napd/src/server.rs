use std::{
    fs, io,
    os::unix::{
        fs::{FileTypeExt, MetadataExt, PermissionsExt},
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
};

use napd_protocol::{
    ASSET_CHUNK_BYTES, MAX_ASSET_BYTES, Request, Response, VERSION, encode_asset_chunk, read_frame,
    write_frame,
};

use crate::{LinuxRunner, RunnerError};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("daemon socket parent could not be prepared: {0}")]
    SocketParent(io::Error),
    #[error("refusing unsafe stale daemon socket at {0}")]
    UnsafeStaleSocket(PathBuf),
    #[error("refusing non-directory or symlink daemon socket parent at {0}")]
    UnsafeSocketParent(PathBuf),
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
pub struct DaemonState {
    runner: LinuxRunner,
    transfer: Option<AssetTransfer>,
}

impl DaemonState {
    pub fn new(runner: LinuxRunner) -> Self {
        Self {
            runner,
            transfer: None,
        }
    }

    pub fn handle(&mut self, request: Request) -> (Response, bool) {
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
                    active_surface: self.runner.active_surface().map(str::to_owned),
                    active_identity,
                },
                Err(error) => runner_error(error),
            },
            Request::StartFixture => match self.runner.start_fixture() {
                Ok(launch) if launch.artifact_html.len() <= MAX_ASSET_BYTES => {
                    let total_bytes = launch.artifact_html.len() as u64;
                    let surface = launch.metadata();
                    let transfer_id = format!("fixture-index-{}", surface.surface_token);
                    self.transfer = Some(AssetTransfer {
                        id: transfer_id.clone(),
                        bytes: launch.artifact_html.into_bytes(),
                        next_offset: 0,
                    });
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
            },
            Request::StopFixture { surface_token } => {
                match self.runner.stop_fixture(&surface_token) {
                    Ok(()) => {
                        self.transfer = None;
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
                Ok(envelope) => Response::Envelope { envelope },
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
        let shutdown = matches!(response, Response::Shutdown);
        (response, shutdown)
    }

    fn asset_chunk(&mut self, transfer_id: &str, offset: u64) -> Response {
        let Some(transfer) = self.transfer.as_mut() else {
            return Response::error("unknown_transfer", "no verified asset transfer is active");
        };
        if transfer.id != transfer_id {
            return Response::error("unknown_transfer", "asset transfer id is not active");
        }
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
    if write_frame(stream, &response).is_err() {
        return false;
    }
    shutdown
}

fn prepare_socket_parent(socket_path: &Path) -> Result<(), ServerError> {
    let parent = socket_path
        .parent()
        .ok_or_else(|| ServerError::UnsafeStaleSocket(socket_path.to_path_buf()))?;
    fs::create_dir_all(parent).map_err(ServerError::SocketParent)?;
    let metadata = fs::symlink_metadata(parent).map_err(ServerError::SocketParent)?;
    if !metadata.file_type().is_dir() {
        return Err(ServerError::UnsafeSocketParent(parent.to_path_buf()));
    }
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700)).map_err(ServerError::Permissions)
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
        let (transfer_id, total_bytes) = match exchange(&socket, &Request::StartFixture) {
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
}
