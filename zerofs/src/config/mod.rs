//! Module for configuration related code.

mod config;
mod default;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use config::*;
pub use default::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroutils_config::default::{DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};
