#![forbid(unsafe_code)]
#![doc = "Bounded version-0 private shell-to-daemon protocol."]

use std::{
    collections::BTreeMap,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub const VERSION: u8 = 0;
/// Uzel's host-side bound for one runtime envelope before IPC serialization.
pub const MAXIMUM_ENVELOPE_BYTES: usize = 64 * 1_024;
/// Private IPC must carry one bounded envelope after worst-case JSON string
/// escaping plus its operation/surface wrapper. A 512 KiB ceiling covers the
/// sixfold `serde_json` control-character expansion of a 64 KiB string while
/// remaining equal to the verified-document bound.
pub const MAX_FRAME_BYTES: usize = 512 * 1_024;
pub const ASSET_CHUNK_BYTES: usize = 2_048;
pub const MAX_ASSET_BYTES: usize = 512 * 1_024;
const IPC_TIMEOUT: Duration = Duration::from_secs(20);
const REPLAYABLE_REQUEST_ATTEMPTS: usize = 2;

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum Request {
    Hello {
        version: u8,
    },
    Status,
    StartFixture {
        fixture: String,
    },
    StartHostileProbe {
        sentinel_url: String,
    },
    ReviewNapplet {
        operation_id: String,
        coordinate: String,
    },
    CancelNappletReview {
        token: String,
    },
    ConfirmNapplet {
        operation_id: String,
        token: String,
        expected_author: String,
        expected_d_tag: String,
        expected_aggregate_hash: String,
        granted_domains: Vec<String>,
    },
    StopFixture {
        surface_token: String,
    },
    AssetChunk {
        transfer_id: String,
        offset: u64,
    },
    ForwardEnvelope {
        surface_token: String,
        envelope: String,
    },
    SetReadIdentity {
        public_identity: String,
    },
    GetReadIdentity,
    Diagnostics,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceMetadata {
    pub surface_token: String,
    pub artifact_base_url: String,
    pub title: String,
    pub author: String,
    pub d_tag: String,
    pub aggregate_hash: String,
    pub domains: Vec<String>,
    pub unavailable_domains: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogCapability {
    pub domain: String,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NappletReview {
    pub token: String,
    pub event_id: String,
    pub coordinate: String,
    pub manifest_author: String,
    pub d_tag: String,
    pub title: String,
    pub description: Option<String>,
    pub aggregate_hash: String,
    pub capabilities: Vec<CatalogCapability>,
    pub blob_sources: Vec<String>,
    pub provenance: Vec<String>,
    pub can_install: bool,
    pub blocker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutedEnvelope {
    pub surface_token: String,
    pub envelope: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayDiagnostic {
    pub relay: String,
    pub access: String,
    pub wire_subscriptions: u64,
    pub authors_served: u64,
    pub lanes: Vec<String>,
    pub events_by_kind: Vec<String>,
    pub nip11_freshness: Option<String>,
    pub nip11_last_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostics {
    pub snapshot_revision: u64,
    pub active_sessions: u64,
    pub active_identity: Option<String>,
    pub relay_revision: u64,
    pub observing_relays: bool,
    pub relays: u64,
    pub omitted_relays: u64,
    pub uncovered_authors: u64,
    pub rejected_private_relays: u64,
    pub sessions_rejected_over_cap: u64,
    pub relay_details: Vec<RelayDiagnostic>,
    pub store_degraded: Option<String>,
    pub transport_degraded: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Response {
    Hello {
        version: u8,
    },
    Status {
        version: u8,
        mode: String,
        active_surfaces: Vec<String>,
        pending_reviews: Vec<String>,
        active_identity: Option<String>,
    },
    Surface {
        surface: SurfaceMetadata,
        transfer_id: String,
        total_bytes: u64,
    },
    NappletReview {
        review: NappletReview,
    },
    ReviewCancelled,
    AssetChunk {
        transfer_id: String,
        offset: u64,
        next_offset: u64,
        total_bytes: u64,
        bytes_base64: String,
        done: bool,
    },
    Envelope {
        surface_token: String,
        envelope: String,
    },
    Identity {
        active_public_key: Option<String>,
    },
    Diagnostics {
        diagnostics: Diagnostics,
    },
    Stopped,
    Shutdown,
    Error {
        code: String,
        detail: String,
    },
}

impl Response {
    pub fn error(code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Error {
            code: code.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("control frame length {actual} exceeds {maximum} bytes")]
    FrameTooLarge { actual: usize, maximum: usize },
    #[error("peer closed before a complete frame arrived")]
    Truncated,
    #[error("control frame is not valid protocol JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("asset chunk offset is out of order: expected {expected}, received {actual}")]
    OutOfOrder { expected: u64, actual: u64 },
    #[error("asset transfer exceeds the {MAX_ASSET_BYTES}-byte limit")]
    AssetTooLarge,
    #[error("asset chunk is not valid base64: {0}")]
    Base64(#[from] base64::DecodeError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FetchedSurface {
    pub surface: SurfaceMetadata,
    pub artifact_bytes: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error("daemon refused {code}: {detail}")]
    Refused { code: String, detail: String },
    #[error("daemon returned an unexpected response")]
    UnexpectedResponse,
    #[error("asset transfer id changed during transfer")]
    TransferChanged,
    #[error("asset chunk length or completion marker is invalid")]
    InvalidChunk,
    #[error(
        "asset transfer for surface {surface_token} failed ({transfer_error}); cleanup also failed ({cleanup_error})"
    )]
    TransferCleanupFailed {
        surface_token: String,
        transfer_error: Box<ClientError>,
        cleanup_error: Box<ClientError>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PendingOperation {
    Review {
        coordinate: String,
    },
    Confirm {
        token: String,
        expected_author: String,
        expected_d_tag: String,
        expected_aggregate_hash: String,
        granted_domains: Vec<String>,
    },
}

#[derive(Clone, Debug)]
pub struct UnixClient {
    socket_path: PathBuf,
    pending_operations: Arc<Mutex<BTreeMap<PendingOperation, String>>>,
}

impl UnixClient {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            pending_operations: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn request(&self, request: &Request) -> Result<Response, ClientError> {
        let mut stream = UnixStream::connect(&self.socket_path).map_err(ProtocolError::Io)?;
        stream
            .set_read_timeout(Some(IPC_TIMEOUT))
            .map_err(ProtocolError::Io)?;
        stream
            .set_write_timeout(Some(IPC_TIMEOUT))
            .map_err(ProtocolError::Io)?;
        write_frame(&mut stream, request)?;
        let response = read_frame(&mut stream)?;
        if let Response::Error { code, detail } = response {
            return Err(ClientError::Refused { code, detail });
        }
        Ok(response)
    }

    pub fn start_fixture(&self) -> Result<FetchedSurface, ClientError> {
        self.start_named_fixture("good-morning")
    }

    pub fn start_named_fixture(&self, fixture: &str) -> Result<FetchedSurface, ClientError> {
        self.fetch_surface(&Request::StartFixture {
            fixture: fixture.to_owned(),
        })
    }

    pub fn start_hostile_probe(&self, sentinel_url: &str) -> Result<FetchedSurface, ClientError> {
        self.fetch_surface(&Request::StartHostileProbe {
            sentinel_url: sentinel_url.to_owned(),
        })
    }

    pub fn review_napplet(&self, coordinate: &str) -> Result<NappletReview, ClientError> {
        let operation = PendingOperation::Review {
            coordinate: coordinate.to_owned(),
        };
        let operation_id = self.operation_id_for(&operation);
        match self.request_replayable(
            &operation,
            &Request::ReviewNapplet {
                operation_id,
                coordinate: coordinate.to_owned(),
            },
        )? {
            Response::NappletReview { review } => Ok(review),
            _ => Err(ClientError::UnexpectedResponse),
        }
    }

    pub fn cancel_napplet_review(&self, token: &str) -> Result<(), ClientError> {
        match self.request(&Request::CancelNappletReview {
            token: token.to_owned(),
        })? {
            Response::ReviewCancelled => Ok(()),
            _ => Err(ClientError::UnexpectedResponse),
        }
    }

    pub fn stop_fixture(&self, surface_token: &str) -> Result<(), ClientError> {
        match self.request(&Request::StopFixture {
            surface_token: surface_token.to_owned(),
        })? {
            Response::Stopped => Ok(()),
            _ => Err(ClientError::UnexpectedResponse),
        }
    }

    pub fn confirm_napplet(
        &self,
        token: &str,
        expected_author: &str,
        expected_d_tag: &str,
        expected_aggregate_hash: &str,
        granted_domains: Vec<String>,
    ) -> Result<FetchedSurface, ClientError> {
        let operation = PendingOperation::Confirm {
            token: token.to_owned(),
            expected_author: expected_author.to_owned(),
            expected_d_tag: expected_d_tag.to_owned(),
            expected_aggregate_hash: expected_aggregate_hash.to_owned(),
            granted_domains: granted_domains.clone(),
        };
        let operation_id = self.operation_id_for(&operation);
        let response = self.request_replayable(
            &operation,
            &Request::ConfirmNapplet {
                operation_id,
                token: token.to_owned(),
                expected_author: expected_author.to_owned(),
                expected_d_tag: expected_d_tag.to_owned(),
                expected_aggregate_hash: expected_aggregate_hash.to_owned(),
                granted_domains,
            },
        )?;
        self.fetch_surface_response(response)
    }

    fn fetch_surface(&self, request: &Request) -> Result<FetchedSurface, ClientError> {
        let response = self.request(request)?;
        self.fetch_surface_response(response)
    }

    fn fetch_surface_response(&self, response: Response) -> Result<FetchedSurface, ClientError> {
        let (surface, transfer_id, total_bytes) = match response {
            Response::Surface {
                surface,
                transfer_id,
                total_bytes,
            } => (surface, transfer_id, total_bytes),
            _ => return Err(ClientError::UnexpectedResponse),
        };
        let artifact_bytes = match self.fetch_asset(&transfer_id, total_bytes) {
            Ok(bytes) => bytes,
            Err(transfer_error) => {
                return match self.stop_fixture(&surface.surface_token) {
                    Ok(()) => Err(transfer_error),
                    Err(cleanup_error) => Err(ClientError::TransferCleanupFailed {
                        surface_token: surface.surface_token,
                        transfer_error: Box::new(transfer_error),
                        cleanup_error: Box::new(cleanup_error),
                    }),
                };
            }
        };
        Ok(FetchedSurface {
            surface,
            artifact_bytes,
        })
    }

    fn operation_id_for(&self, operation: &PendingOperation) -> String {
        let mut pending = self
            .pending_operations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        pending
            .entry(operation.clone())
            .or_insert_with(next_operation_id)
            .clone()
    }

    fn request_replayable(
        &self,
        operation: &PendingOperation,
        request: &Request,
    ) -> Result<Response, ClientError> {
        for attempt in 0..REPLAYABLE_REQUEST_ATTEMPTS {
            match self.request(request) {
                Ok(response) => {
                    self.forget_operation(operation);
                    return Ok(response);
                }
                Err(ClientError::Protocol(error)) if attempt + 1 == REPLAYABLE_REQUEST_ATTEMPTS => {
                    return Err(ClientError::Protocol(error));
                }
                Err(ClientError::Protocol(_)) => {}
                Err(error) => {
                    self.forget_operation(operation);
                    return Err(error);
                }
            }
        }
        unreachable!("replayable request attempt bound is nonzero")
    }

    fn forget_operation(&self, operation: &PendingOperation) {
        self.pending_operations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(operation);
    }

    fn fetch_asset(&self, transfer_id: &str, total_bytes: u64) -> Result<Vec<u8>, ClientError> {
        let total = usize::try_from(total_bytes).map_err(|_| ProtocolError::AssetTooLarge)?;
        if total == 0 || total > MAX_ASSET_BYTES {
            return Err(ProtocolError::AssetTooLarge.into());
        }
        let mut artifact_bytes = Vec::with_capacity(total);
        let mut offset = 0_u64;
        while offset < total_bytes {
            let response = self.request(&Request::AssetChunk {
                transfer_id: transfer_id.to_owned(),
                offset,
            })?;
            let Response::AssetChunk {
                transfer_id: returned_transfer,
                offset: returned_offset,
                next_offset,
                total_bytes: returned_total,
                bytes_base64,
                done,
            } = response
            else {
                return Err(ClientError::UnexpectedResponse);
            };
            if returned_transfer != transfer_id {
                return Err(ClientError::TransferChanged);
            }
            if returned_offset != offset {
                return Err(ProtocolError::OutOfOrder {
                    expected: offset,
                    actual: returned_offset,
                }
                .into());
            }
            if returned_total != total_bytes {
                return Err(ClientError::InvalidChunk);
            }
            let chunk = decode_asset_chunk(&bytes_base64)?;
            let chunk_length = u64::try_from(chunk.len()).map_err(|_| ClientError::InvalidChunk)?;
            if chunk.is_empty()
                || chunk.len() > ASSET_CHUNK_BYTES
                || next_offset != offset.saturating_add(chunk_length)
                || next_offset > total_bytes
                || done != (next_offset == total_bytes)
            {
                return Err(ClientError::InvalidChunk);
            }
            artifact_bytes.extend(chunk);
            offset = next_offset;
        }
        if artifact_bytes.len() != total {
            return Err(ClientError::InvalidChunk);
        }
        Ok(artifact_bytes)
    }
}

fn next_operation_id() -> String {
    let sequence = NEXT_OPERATION_ID.fetch_add(1, Ordering::Relaxed);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}-{timestamp:x}-{sequence:x}", std::process::id())
}

pub fn write_frame<T: Serialize>(writer: &mut impl Write, value: &T) -> Result<(), ProtocolError> {
    let frame = serde_json::to_vec(value)?;
    if frame.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge {
            actual: frame.len(),
            maximum: MAX_FRAME_BYTES,
        });
    }
    writer.write_all(&(frame.len() as u32).to_be_bytes())?;
    writer.write_all(&frame)?;
    writer.flush()?;
    Ok(())
}

pub fn read_frame<T: DeserializeOwned>(reader: &mut impl Read) -> Result<T, ProtocolError> {
    let mut prefix = [0_u8; 4];
    read_exact_or_truncated(reader, &mut prefix)?;
    let length = u32::from_be_bytes(prefix) as usize;
    if length > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge {
            actual: length,
            maximum: MAX_FRAME_BYTES,
        });
    }
    let mut frame = vec![0_u8; length];
    read_exact_or_truncated(reader, &mut frame)?;
    Ok(serde_json::from_slice(&frame)?)
}

pub fn encode_asset_chunk(bytes: &[u8]) -> String {
    BASE64.encode(bytes)
}

pub fn decode_asset_chunk(encoded: &str) -> Result<Vec<u8>, ProtocolError> {
    Ok(BASE64.decode(encoded)?)
}

fn read_exact_or_truncated(reader: &mut impl Read, bytes: &mut [u8]) -> Result<(), ProtocolError> {
    reader.read_exact(bytes).map_err(|error| {
        if error.kind() == std::io::ErrorKind::UnexpectedEof {
            ProtocolError::Truncated
        } else {
            ProtocolError::Io(error)
        }
    })
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Cursor,
        os::unix::net::UnixListener,
        sync::atomic::{AtomicU64, Ordering},
        thread,
    };

    use super::*;

    static NEXT_SOCKET: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn gate_zero_bounds_remain_exact() {
        assert_eq!(VERSION, 0);
        assert_eq!(MAXIMUM_ENVELOPE_BYTES, 64 * 1_024);
        assert_eq!(MAX_FRAME_BYTES, 512 * 1_024);
    }

    #[test]
    fn frames_round_trip_with_big_endian_length() {
        let request = Request::Hello { version: VERSION };
        let mut wire = Vec::new();
        write_frame(&mut wire, &request).unwrap();
        assert_eq!(
            u32::from_be_bytes(wire[..4].try_into().unwrap()) as usize,
            wire.len() - 4
        );
        assert_eq!(
            read_frame::<Request>(&mut Cursor::new(wire)).unwrap(),
            request
        );
    }

    #[test]
    fn oversized_frame_is_rejected_before_body_read() {
        let mut wire = Cursor::new(((MAX_FRAME_BYTES + 1) as u32).to_be_bytes());
        assert!(matches!(
            read_frame::<Request>(&mut wire),
            Err(ProtocolError::FrameTooLarge { .. })
        ));
    }

    #[test]
    fn maximum_asset_chunk_fits_control_frame() {
        let response = Response::AssetChunk {
            transfer_id: "fixture-index-generation-1".to_owned(),
            offset: 500_000,
            next_offset: 502_048,
            total_bytes: MAX_ASSET_BYTES as u64,
            bytes_base64: encode_asset_chunk(&vec![255; ASSET_CHUNK_BYTES]),
            done: false,
        };
        let mut wire = Vec::new();
        write_frame(&mut wire, &response).unwrap();
        assert!(wire.len() <= MAX_FRAME_BYTES + 4);
    }

    #[test]
    fn maximum_envelope_with_worst_case_json_escaping_fits_control_frame() {
        let envelope = "\0".repeat(MAXIMUM_ENVELOPE_BYTES);
        let request = Request::ForwardEnvelope {
            surface_token: "s".repeat(128),
            envelope: envelope.clone(),
        };
        let response = Response::Envelope {
            surface_token: "s".repeat(128),
            envelope,
        };
        for frame_length in [
            serde_json::to_vec(&request).unwrap().len(),
            serde_json::to_vec(&response).unwrap().len(),
        ] {
            assert!(frame_length > 128 * 1_024);
            assert!(frame_length <= MAX_FRAME_BYTES);
        }
        let mut request_wire = Vec::new();
        write_frame(&mut request_wire, &request).unwrap();
        let mut response_wire = Vec::new();
        write_frame(&mut response_wire, &response).unwrap();
        for wire in [request_wire, response_wire] {
            assert!(wire.len() <= MAX_FRAME_BYTES + 4);
        }
    }

    #[test]
    fn failed_asset_transfer_stops_the_launched_surface() {
        let unique = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "uzel-napd-protocol-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let socket = root.join("napd.sock");
        let listener = UnixListener::bind(&socket).unwrap();
        let server = thread::spawn(move || {
            for exchange in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_frame::<Request>(&mut stream).unwrap();
                let response = match (exchange, request) {
                    (0, Request::StartFixture { .. }) => Response::Surface {
                        surface: SurfaceMetadata {
                            surface_token: "surface-after-launch".to_owned(),
                            artifact_base_url:
                                "nmp-artifact://00000000-0000-4000-8000-000000000001/".to_owned(),
                            title: "Test".to_owned(),
                            author: "a".repeat(64),
                            d_tag: "test".to_owned(),
                            aggregate_hash: "b".repeat(64),
                            domains: Vec::new(),
                            unavailable_domains: Vec::new(),
                        },
                        transfer_id: "transfer-after-launch".to_owned(),
                        total_bytes: 1,
                    },
                    (1, Request::AssetChunk { .. }) => Response::Stopped,
                    (2, Request::StopFixture { surface_token }) => {
                        assert_eq!(surface_token, "surface-after-launch");
                        Response::Stopped
                    }
                    (_, request) => panic!("unexpected request: {request:?}"),
                };
                write_frame(&mut stream, &response).unwrap();
            }
        });

        let error = UnixClient::new(&socket).start_fixture().unwrap_err();
        assert!(matches!(error, ClientError::UnexpectedResponse));
        server.join().unwrap();
        fs::remove_file(&socket).unwrap();
        fs::remove_dir(&root).unwrap();
    }

    #[test]
    fn review_retry_reuses_operation_after_responses_are_lost() {
        let unique = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "uzel-napd-protocol-review-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let socket = root.join("napd.sock");
        let listener = UnixListener::bind(&socket).unwrap();
        let server = thread::spawn(move || {
            let mut first_request = None;
            for exchange in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_frame::<Request>(&mut stream).unwrap();
                match &first_request {
                    Some(first) => assert_eq!(&request, first),
                    None => first_request = Some(request.clone()),
                }
                let Request::ReviewNapplet {
                    operation_id,
                    coordinate,
                } = request
                else {
                    panic!("unexpected request")
                };
                assert!(!operation_id.is_empty());
                assert_eq!(coordinate, "naddr-test");
                if exchange < 2 {
                    continue;
                }
                write_frame(
                    &mut stream,
                    &Response::NappletReview {
                        review: NappletReview {
                            token: "review-token".to_owned(),
                            event_id: "e".repeat(64),
                            coordinate,
                            manifest_author: "a".repeat(64),
                            d_tag: "test".to_owned(),
                            title: "Test".to_owned(),
                            description: None,
                            aggregate_hash: "b".repeat(64),
                            capabilities: Vec::new(),
                            blob_sources: Vec::new(),
                            provenance: Vec::new(),
                            can_install: true,
                            blocker: None,
                        },
                    },
                )
                .unwrap();
            }
        });

        let client = UnixClient::new(&socket);
        assert!(matches!(
            client.review_napplet("naddr-test"),
            Err(ClientError::Protocol(_))
        ));
        assert_eq!(
            client.review_napplet("naddr-test").unwrap().token,
            "review-token"
        );
        server.join().unwrap();
        fs::remove_file(&socket).unwrap();
        fs::remove_dir(&root).unwrap();
    }

    #[test]
    fn confirm_response_loss_replays_surface_without_a_second_operation() {
        let unique = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "uzel-napd-protocol-confirm-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let socket = root.join("napd.sock");
        let listener = UnixListener::bind(&socket).unwrap();
        let server = thread::spawn(move || {
            let (mut first_stream, _) = listener.accept().unwrap();
            let first = read_frame::<Request>(&mut first_stream).unwrap();
            let Request::ConfirmNapplet {
                ref operation_id, ..
            } = first
            else {
                panic!("unexpected request")
            };
            assert!(!operation_id.is_empty());
            drop(first_stream);

            let (mut replay_stream, _) = listener.accept().unwrap();
            let replay = read_frame::<Request>(&mut replay_stream).unwrap();
            assert_eq!(replay, first);
            write_frame(
                &mut replay_stream,
                &Response::Surface {
                    surface: SurfaceMetadata {
                        surface_token: "confirmed-surface".to_owned(),
                        artifact_base_url: "nmp-artifact://00000000-0000-4000-8000-000000000001/"
                            .to_owned(),
                        title: "Test".to_owned(),
                        author: "a".repeat(64),
                        d_tag: "test".to_owned(),
                        aggregate_hash: "b".repeat(64),
                        domains: Vec::new(),
                        unavailable_domains: Vec::new(),
                    },
                    transfer_id: "confirmed-transfer".to_owned(),
                    total_bytes: 1,
                },
            )
            .unwrap();

            let (mut asset_stream, _) = listener.accept().unwrap();
            assert_eq!(
                read_frame::<Request>(&mut asset_stream).unwrap(),
                Request::AssetChunk {
                    transfer_id: "confirmed-transfer".to_owned(),
                    offset: 0,
                }
            );
            write_frame(
                &mut asset_stream,
                &Response::AssetChunk {
                    transfer_id: "confirmed-transfer".to_owned(),
                    offset: 0,
                    next_offset: 1,
                    total_bytes: 1,
                    bytes_base64: encode_asset_chunk(b"x"),
                    done: true,
                },
            )
            .unwrap();
        });

        let fetched = UnixClient::new(&socket)
            .confirm_napplet(
                "review-token",
                &"a".repeat(64),
                "test",
                &"b".repeat(64),
                Vec::new(),
            )
            .unwrap();
        assert_eq!(fetched.surface.surface_token, "confirmed-surface");
        assert_eq!(fetched.artifact_bytes, b"x");
        server.join().unwrap();
        fs::remove_file(&socket).unwrap();
        fs::remove_dir(&root).unwrap();
    }
}
