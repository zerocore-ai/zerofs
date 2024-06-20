use serde::{Deserialize, Serialize};
use structstruck::strike;
use typed_builder::TypedBuilder;
use zeroutils_config::{network::NetworkConfig, ConfigResult, MainConfig};

use super::FsPortDefaults;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

strike! {
    /// Configuration for the zerofs.
    #[strikethrough[derive(Debug, Deserialize, Serialize, TypedBuilder, Default)]]
    pub struct ZerofsConfig {
        /// Network configuration.
        #[serde(default)]
        #[builder(default)]
        pub network: ZerofsNetworkConfig,

        // /// Interface configuration.
        // pub interface: pub struct InterfaceConfig {
        //     /// Base path for the zerofs.
        //     pub base: PathBuf,
        // }
    }
}

/// Network configuration for the zerofs service.
pub type ZerofsNetworkConfig = NetworkConfig<'static, FsPortDefaults>;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl MainConfig for ZerofsConfig {
    fn validate(&self) -> ConfigResult<()> {
        self.network.validate()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        net::{IpAddr, Ipv4Addr, SocketAddr},
        str::FromStr,
    };

    use zeroutils_config::default::{DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};
    use zeroutils_did_wk::WrappedDidWebKey;

    use super::*;

    #[test]
    fn test_toml_full() -> anyhow::Result<()> {
        let toml = r#"
        [network]
        id = "did:wk:z6MkoVs2h6TnfyY8fx2ZqpREWSLS8rBDQmGpyXgFpg63CSUb"
        name = "alice"
        host = "127.0.0.1"
        user_port = 6600
        peer_port = 6611

        [network.seeds]
        "did:wk:m7QFAoSJPFzmaqQiTkLrWQ6pbYrmI6L07Fkdg8SCRpjP1Ig" = "127.0.0.1:7800"
        "did:wk:z6MknLif7jhwt6jUfn14EuDnxWoSHkkajyDi28QMMH5eS1DL" = "127.0.0.1:7900"

        [network.consensus]
        heartbeat_interval = 1000
        election_timeout_range = [150, 300]
        "#;

        let config: ZerofsConfig = toml::from_str(toml)?;

        assert_eq!(
            config.network.id,
            WrappedDidWebKey::from_str("did:wk:z6MkoVs2h6TnfyY8fx2ZqpREWSLS8rBDQmGpyXgFpg63CSUb")?
        );
        assert_eq!(config.network.host, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(config.network.user_port, 6600);
        assert_eq!(config.network.peer_port, 6611);
        assert_eq!(config.network.seeds, {
            let mut peers = HashMap::new();
            peers.insert(
                WrappedDidWebKey::from_str(
                    "did:wk:m7QFAoSJPFzmaqQiTkLrWQ6pbYrmI6L07Fkdg8SCRpjP1Ig",
                )?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7800),
            );
            peers.insert(
                WrappedDidWebKey::from_str(
                    "did:wk:z6MknLif7jhwt6jUfn14EuDnxWoSHkkajyDi28QMMH5eS1DL",
                )?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7900),
            );
            peers
        });
        assert_eq!(config.network.consensus.heartbeat_interval, 1000);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));

        Ok(())
    }

    #[test]
    fn test_toml_defaults() -> anyhow::Result<()> {
        let config: ZerofsConfig = toml::from_str("")?;

        assert_eq!(config.network.host, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(config.network.user_port, 6600);
        assert_eq!(config.network.peer_port, 6611);
        assert!(config.network.seeds.is_empty());
        assert_eq!(
            config.network.consensus.heartbeat_interval,
            DEFAULT_HEARTBEAT_INTERVAL
        );
        assert_eq!(
            config.network.consensus.election_timeout_range,
            DEFAULT_ELECTION_TIMEOUT_RANGE
        );

        Ok(())
    }
}
