use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{constant, FsError, FsResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A path in the file system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Path {
    /// The segments of the path.
    pub segments: Vec<PathSegment>,
}

/// A segment of a path.
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct PathSegment(String);

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Path {
    /// Creates a path from an iterator of path segments.
    pub fn from_iter<T>(
        iter: impl IntoIterator<Item = T>,
    ) -> Result<Self, <T as TryInto<PathSegment>>::Error>
    where
        T: TryInto<PathSegment>,
    {
        let segments = iter
            .into_iter()
            .map(T::try_into)
            .collect::<Result<Vec<_>, <T as TryInto<PathSegment>>::Error>>()?;

        Ok(Self { segments })
    }
}

impl PathSegment {
    /// Validates a path segment.
    pub fn validate(segment: &str) -> FsResult<()> {
        if !RE_VALID_PATH_SEGMENT.is_match(segment) {
            return Err(FsError::InvalidPathSegment(segment.to_owned()));
        }

        Ok(())
    }

    /// Canonicalizes a path segment.
    pub fn canonicalize(&self) -> PathSegment {
        PathSegment(self.0.to_lowercase())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl FromStr for Path {
    type Err = FsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl TryFrom<&str> for Path {
    type Error = FsError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let segments = path
            .split(constant::PATH_SEPARATOR)
            .filter(|segment| !segment.is_empty())
            .map(|segment| {
                PathSegment::validate(segment)?;
                Ok(PathSegment(segment.to_string()))
            })
            .collect::<FsResult<Vec<_>>>()?;

        Ok(Self { segments })
    }
}

impl TryFrom<String> for Path {
    type Error = FsError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        path.as_str().try_into()
    }
}

impl From<Path> for String {
    fn from(path: Path) -> Self {
        path.segments
            .iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(&constant::PATH_SEPARATOR.to_string())
    }
}

impl From<&Path> for String {
    fn from(path: &Path) -> Self {
        path.segments
            .iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(&constant::PATH_SEPARATOR.to_string())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}",
            self.segments
                .iter()
                .map(String::from)
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

impl From<PathSegment> for String {
    fn from(segment: PathSegment) -> Self {
        segment.0
    }
}

impl From<&PathSegment> for String {
    fn from(segment: &PathSegment) -> Self {
        segment.0.clone()
    }
}

impl PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        self.canonicalize().0 == other.canonicalize().0
    }
}

impl Eq for PathSegment {}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RE_VALID_PATH_SEGMENT: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
}
