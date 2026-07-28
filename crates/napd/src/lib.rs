#![forbid(unsafe_code)]
#![doc = "Linux daemon composition boundary for the Uzel POC."]

pub const PROCESS_ROLE: &str = "runtime-authority";

pub use napd_protocol::{MAX_FRAME_BYTES, VERSION as PROTOCOL_VERSION};
