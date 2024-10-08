use serde::{Deserialize, Serialize};
use zeroutils_config::network::PortDefaults;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Port defaults for the file system service.
#[derive(Debug, Deserialize, Serialize)]
pub struct FsPortDefaults;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl PortDefaults for FsPortDefaults {
    fn default_user_port() -> u16 {
        6600
    }

    fn default_peer_port() -> u16 {
        6611
    }
}
