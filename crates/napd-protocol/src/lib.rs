#![forbid(unsafe_code)]
#![doc = "Bounded version-0 private shell-to-daemon protocol."]

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    time::Duration,
};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub const VERSION: u8 = 0;
/// Private IPC must carry one bounded NAP envelope plus its JSON wrapper.
/// Identity projections for ordinary follow lists exceed the old 4 KiB proof
/// value, while the runtime itself already caps envelopes at 64 KiB.
pub const MAX_FRAME_BYTES: usize = 128 * 1_024;
pub const ASSET_CHUNK_BYTES: usize = 2_048;
pub const MAX_ASSET_BYTES: usize = 512 * 1_024;
const IPC_TIMEOUT: Duration = Duration::from_secs(5);

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
pub struct RoutedEnvelope {
    pub surface_token: String,
    pub envelope: String,
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
        active_identity: Option<String>,
    },
    Surface {
        surface: SurfaceMetadata,
        transfer_id: String,
        total_bytes: u64,
    },
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
}

#[derive(Clone, Debug)]
pub struct UnixClient {
    socket_path: PathBuf,
}

impl UnixClient {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
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

    fn fetch_surface(&self, request: &Request) -> Result<FetchedSurface, ClientError> {
        let (surface, transfer_id, total_bytes) = match self.request(request)? {
            Response::Surface {
                surface,
                transfer_id,
                total_bytes,
            } => (surface, transfer_id, total_bytes),
            _ => return Err(ClientError::UnexpectedResponse),
        };
        let total = usize::try_from(total_bytes).map_err(|_| ProtocolError::AssetTooLarge)?;
        if total == 0 || total > MAX_ASSET_BYTES {
            return Err(ProtocolError::AssetTooLarge.into());
        }
        let mut artifact_bytes = Vec::with_capacity(total);
        let mut offset = 0_u64;
        while offset < total_bytes {
            let response = self.request(&Request::AssetChunk {
                transfer_id: transfer_id.clone(),
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
        Ok(FetchedSurface {
            surface,
            artifact_bytes,
        })
    }
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
    use std::io::Cursor;

    use super::*;

    #[test]
    fn gate_zero_bounds_remain_exact() {
        assert_eq!(VERSION, 0);
        assert_eq!(MAX_FRAME_BYTES, 128 * 1_024);
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
}
