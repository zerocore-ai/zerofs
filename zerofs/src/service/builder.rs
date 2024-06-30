use std::sync::Arc;

use zeroutils_config::{network::NetworkConfig, MainConfig};
use zeroutils_did_wk::{Base, WrappedDidWebKey};
use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;

use crate::{config::ZerofsConfig, filesystem::Dir};

use super::{FsService, ServiceResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A builder for the file system service.
pub struct FsServiceBuilder<'a, S = (), K = ()> {
    store: S,
    key: &'a K,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a, S, K> FsServiceBuilder<'a, S, K> {
    /// Sets the block store for the file system service
    pub fn store<T>(self, store: T) -> FsServiceBuilder<'a, T, K>
    where
        T: IpldStore,
    {
        FsServiceBuilder {
            store,
            key: self.key,
        }
    }

    /// Sets the key pair to be used to manage the file system service.
    pub fn key<T>(self, key: &'a T) -> FsServiceBuilder<'a, S, T> {
        FsServiceBuilder {
            store: self.store,
            key,
        }
    }
}

impl<'a, S, K> FsServiceBuilder<'a, S, K>
where
    S: IpldStore + Send + Sync,
    K: GetPublicKey,
{
    /// Builds the file system service.
    pub fn build(self) -> ServiceResult<FsService<S>> {
        let did = WrappedDidWebKey::from_key(self.key, Base::Base58Btc)?;

        let config = ZerofsConfig {
            network: NetworkConfig::builder().id(did).build(),
            // interface: InterfaceConfig::builder().build(),
        };

        config.validate()?;

        let service = FsService {
            root_dir: Dir::new(self.store),
            config: Arc::new(config),
        };

        Ok(service)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a> Default for FsServiceBuilder<'a> {
    fn default() -> Self {
        FsServiceBuilder {
            store: (),
            key: &(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use zeroutils_key::{Ed25519KeyPair, KeyPairGenerate};
    use zeroutils_store::MemoryStore;

    use super::*;

    #[test]
    fn test_fs_service_builder() -> anyhow::Result<()> {
        let keypair = Ed25519KeyPair::generate(&mut rand::thread_rng())?;
        let store = MemoryStore::default();

        let _fs_service = FsServiceBuilder::default()
            .store(store)
            .key(&keypair)
            .build()?;

        // TODO: Add tests for the file system service

        Ok(())
    }
}
