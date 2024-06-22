use async_once_cell::OnceCell;
use zeroutils_store::{ipld::cid::Cid, IpldStore, Storable};

use super::{Entity, FsResult, Path};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a link in the filesystem, which points to an entity.
pub struct Link<L, S>
where
    S: IpldStore,
{
    /// The link representation to the target entity.
    /// This could be a CID or a path depending on the link type.
    link: L,

    /// The cached entity associated with the link.
    cached_entity: CachedEntity<S>,
}

/// A type alias for `OnceCell` holding a lazily initialized `Entity`.
///
/// This is used to cache an entity associated with a `Link`, ensuring that the entity is only
/// loaded once from the underlying store when accessed.
pub type CachedEntity<S> = OnceCell<Entity<S>>;

/// A type alias for a link to an entity in the filesystem, using a CID.
pub type CidLink<S> = Link<Cid, S>;

/// A type alias for a link to an entity in the filesystem, using a path.
pub type PathLink<S> = Link<Path, S>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<L, S> Link<L, S>
where
    S: IpldStore,
{
    /// Gets the entity associated with the link, if it exists.
    pub fn cached_entity(&self) -> Option<&Entity<S>> {
        self.cached_entity.get()
    }
}

impl<S> CidLink<S>
where
    S: IpldStore,
{
    /// Gets the CID link.
    pub fn cid(&self) -> &Cid {
        &self.link
    }

    /// Resolves the link to an entity, loading it from the store if necessary.
    pub async fn resolve_entity(&self, store: S) -> FsResult<&Entity<S>> {
        self.cached_entity
            .get_or_try_init(Entity::load(&self.link, store))
            .await
            .map_err(Into::into)
    }
}

impl<S> PathLink<S>
where
    S: IpldStore,
{
    /// Gets the path link.
    pub fn path(&self) -> &Path {
        &self.link
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> From<Cid> for CidLink<S>
where
    S: IpldStore,
{
    fn from(cid: Cid) -> Self {
        Self {
            link: cid,
            cached_entity: OnceCell::new(),
        }
    }
}

impl<S> From<Path> for PathLink<S>
where
    S: IpldStore,
{
    fn from(path: Path) -> Self {
        Self {
            link: path,
            cached_entity: OnceCell::new(),
        }
    }
}

impl<L, S> Clone for Link<L, S>
where
    S: IpldStore,
    L: Clone,
{
    fn clone(&self) -> Self {
        Self {
            link: self.link.clone(),
            cached_entity: OnceCell::new(),
        }
    }
}

impl<L, S> PartialEq for Link<L, S>
where
    S: IpldStore,
    L: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
}
