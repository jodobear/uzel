#![forbid(unsafe_code)]
#![doc = "Bounded version-0 private shell-to-daemon protocol."]

use std::io::{Read, Write};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub const VERSION: u8 = 0;
pub const MAX_FRAME_BYTES: usize = 4_096;
pub const ASSET_CHUNK_BYTES: usize = 2_048;
pub const MAX_ASSET_BYTES: usize = 512 * 1_024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum Request {
    Hello {
        version: u8,
    },
    Status,
    StartFixture,
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
        active_surface: Option<String>,
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
        assert_eq!(MAX_FRAME_BYTES, 4_096);
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
