#![forbid(unsafe_code)]
#![doc = "Linux daemon composition boundary for the Uzel POC."]

mod runner;

pub const PROCESS_ROLE: &str = "runtime-authority";

pub use napd_protocol::{MAX_FRAME_BYTES, VERSION as PROTOCOL_VERSION};
pub use runner::{LinuxRunner, RunnerError, SurfaceLaunch};
