use async_once_cell::OnceCell;
use zeroutils_store::IpldStore;

use crate::filesystem::{Entity, Path};

use super::Link;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A link representing an association between a [`Path`] and some lazily loaded value.
pub type PathLink<T> = Link<Path, T>;

/// A link representing an association between a [`Path`] and a lazily loaded [`Entity`].
pub type EntityPathLink<S> = PathLink<Entity<S>>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<T> PathLink<T> {
    /// Gets the path of the link.
    pub fn get_path(&self) -> &Path {
        &self.identifier
    }
}

impl<S> EntityPathLink<S>
where
    S: IpldStore,
{
    /// Change the store used to persist the path link.
    pub fn use_store<T>(self, _: &T) -> EntityPathLink<T>
    where
        T: IpldStore,
    {
        EntityPathLink {
            identifier: self.identifier,
            cached: OnceCell::new(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T> From<Path> for PathLink<T> {
    fn from(path: Path) -> Self {
        Self {
            identifier: path,
            cached: OnceCell::new(),
        }
    }
}
