use std::{
    fmt::{self, Debug},
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use zeroutils_store::IpldStore;

use super::{Dir, PathSegment};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A collection of directories and their corresponding names in their respective parent directories.
/// For example, if the path is `/a/b/c`, the pathdirs will hold the directories representing `a`, `b`,
/// and `c` along with those names.
#[derive(Clone)]
pub struct PathDirs<S>
where
    S: IpldStore,
{
    path: Vec<(Dir<S>, PathSegment)>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> PathDirs<S>
where
    S: IpldStore,
{
    /// Create a new empty `PathDirs`.
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    /// Returns the number of segments in the path.
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Returns whether the path is empty.
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Returns an iterator over the path segments.
    pub fn iter(&self) -> impl Iterator<Item = &(Dir<S>, PathSegment)> {
        self.path.iter()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> FromIterator<(Dir<S>, PathSegment)> for PathDirs<S>
where
    S: IpldStore,
{
    fn from_iter<I: IntoIterator<Item = (Dir<S>, PathSegment)>>(iter: I) -> Self {
        Self {
            path: iter.into_iter().collect(),
        }
    }
}

impl<S> IntoIterator for PathDirs<S>
where
    S: IpldStore,
{
    type Item = (Dir<S>, PathSegment);
    type IntoIter = <Vec<(Dir<S>, PathSegment)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<S> Extend<(Dir<S>, PathSegment)> for PathDirs<S>
where
    S: IpldStore,
{
    fn extend<T: IntoIterator<Item = (Dir<S>, PathSegment)>>(&mut self, iter: T) {
        self.path.extend(iter);
    }
}

impl<S> Deref for PathDirs<S>
where
    S: IpldStore,
{
    type Target = Vec<(Dir<S>, PathSegment)>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<S> DerefMut for PathDirs<S>
where
    S: IpldStore,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl<S> Default for PathDirs<S>
where
    S: IpldStore,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Debug for PathDirs<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.path.iter()).finish()
    }
}
