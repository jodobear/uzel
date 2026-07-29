#![forbid(unsafe_code)]
#![doc = "Linux daemon composition boundary for the Uzel POC."]

mod fixtures;
mod runner;
mod server;

pub const PROCESS_ROLE: &str = "runtime-authority";

pub use napd_protocol::{MAX_FRAME_BYTES, VERSION as PROTOCOL_VERSION};
pub use runner::{LinuxRunner, RunnerError, RuntimeMode, SurfaceLaunch};
pub use server::{DaemonServer, DaemonState, ServerError};
