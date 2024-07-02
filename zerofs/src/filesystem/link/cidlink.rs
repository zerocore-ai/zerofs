use async_once_cell::OnceCell;
use zeroutils_store::{ipld::cid::Cid, IpldStore, Storable};

use crate::filesystem::{Entity, FsResult};

use super::{Link, Resolvable};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A link representing an association between [`Cid`] and some lazily loaded value.
pub type CidLink<T> = Link<Cid, T>;

/// A link representing an association between [`Cid`] and a lazily loaded [`Entity`].
pub type EntityCidLink<S> = CidLink<Entity<S>>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<T> CidLink<T> {
    /// Gets the CID of the link.
    pub fn cid(&self) -> &Cid {
        &self.identifier
    }
}

impl<S> EntityCidLink<S>
where
    S: IpldStore,
{
    /// Change the store used to persist the CID link.
    pub fn use_store<T>(self, _: &T) -> EntityCidLink<T>
    where
        T: IpldStore,
    {
        EntityCidLink {
            identifier: self.identifier,
            cached: OnceCell::new(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, S> Resolvable<'a, S> for EntityCidLink<S>
where
    S: IpldStore + Send + Sync + 'a,
{
    type Target = Entity<S>;

    /// Resolves the [`EntityCidLink`] to an [`Entity`].
    async fn resolve(&'a self, store: S) -> FsResult<&'a Self::Target> {
        self.cached
            .get_or_try_init(Entity::load(&self.identifier, store))
            .await
            .map_err(Into::into)
    }
}

impl<T> From<Cid> for CidLink<T> {
    fn from(cid: Cid) -> Self {
        Self {
            identifier: cid,
            cached: OnceCell::new(),
        }
    }
}
