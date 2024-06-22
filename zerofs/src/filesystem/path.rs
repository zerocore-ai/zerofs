use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{FsError, FsResult};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The path separator.
pub const PATH_SEPARATOR: char = '/';

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A path in the file system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Path {
    /// The segments of the path.
    pub segments: Vec<PathSegment>,
}

/// A segment of a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathSegment {
    /// A path segment representing the current directory.
    /// This is represented by a single dot `.`.
    CurrentDir,

    /// A path segment representing the parent directory.
    /// This is represented by a double dot `..`.
    ParentDir,

    /// A path segment representing a named directory or file.
    Named(String),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Path {
    /// Creates a path from an iterator of path segments.
    pub fn try_from_iter<T>(
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

    /// Returns the segments of the path.
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Canonicalizes the path by trying to remove all `.` and `..` from the path.
    ///
    /// Leading `.` and `..` that go past the root segment are not supported.
    pub fn canonicalize(&self) -> FsResult<Self> {
        let mut resolved_segments = Vec::new();

        for (i, segment) in self.segments.iter().enumerate() {
            match segment {
                PathSegment::CurrentDir => {
                    if i == 0 {
                        return Err(FsError::LeadingCurrentDir);
                    }
                    // Skip the current directory segment otherwise
                }
                PathSegment::ParentDir => {
                    // Remove the preceding segment unless out of bounds
                    if resolved_segments.is_empty() {
                        return Err(FsError::OutOfBoundsParentDir);
                    }
                    resolved_segments.pop();
                }
                PathSegment::Named(name) => {
                    resolved_segments.push(PathSegment::Named(name.clone()));
                }
            }
        }

        Ok(Self {
            segments: resolved_segments,
        })
    }

    /// Returns the number of segments in the path.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether the path is empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns an iterator over the path segments.
    pub fn iter(&self) -> impl Iterator<Item = &PathSegment> {
        self.segments.iter()
    }

    /// Returns the last segment of the path.
    pub(crate) fn split_last(&self) -> (&[PathSegment], &PathSegment) {
        let (last, init) = self.segments.split_last().unwrap();
        (init, last)
    }
}

impl PathSegment {
    /// Validates a path segment.
    pub fn validate(segment: &str) -> FsResult<()> {
        if segment == "." || segment == ".." {
            return Ok(());
        }

        if !RE_VALID_PATH_SEGMENT.is_match(segment) {
            return Err(FsError::InvalidPathSegment(segment.to_owned()));
        }

        Ok(())
    }

    /// Canonicalizes a path segment.
    pub fn canonicalize(&self) -> PathSegment {
        match self {
            PathSegment::Named(segment) => PathSegment::Named(segment.to_lowercase()),
            _ => self.clone(),
        }
    }

    /// Returns whether the path segment is a named segment.
    pub fn is_named(&self) -> bool {
        matches!(self, PathSegment::Named(_))
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: Path
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
            .split(PATH_SEPARATOR)
            .filter(|segment| !segment.is_empty())
            .map(PathSegment::try_from)
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

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}",
            self.segments
                .iter()
                .map(|segment| segment.to_string())
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: PathSegment
//--------------------------------------------------------------------------------------------------

impl TryFrom<String> for PathSegment {
    type Error = FsError;

    fn try_from(segment: String) -> Result<Self, Self::Error> {
        PathSegment::validate(&segment)?;
        match segment.as_str() {
            "." => Ok(PathSegment::CurrentDir),
            ".." => Ok(PathSegment::ParentDir),
            _ => Ok(PathSegment::Named(segment)),
        }
    }
}

impl TryFrom<&str> for PathSegment {
    type Error = FsError;

    fn try_from(segment: &str) -> Result<Self, Self::Error> {
        segment.to_string().try_into()
    }
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSegment::CurrentDir => write!(f, "."),
            PathSegment::ParentDir => write!(f, ".."),
            PathSegment::Named(segment) => write!(f, "{}", segment),
        }
    }
}

impl PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self.canonicalize(), other.canonicalize()) {
            (PathSegment::CurrentDir, PathSegment::CurrentDir) => true,
            (PathSegment::ParentDir, PathSegment::ParentDir) => true,
            (PathSegment::Named(a), PathSegment::Named(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for PathSegment {}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RE_VALID_PATH_SEGMENT: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_constructor() -> anyhow::Result<()> {
        let path = Path::try_from_iter(vec!["a", "b", "c"])?;
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0], PathSegment::Named("a".to_owned()));
        assert_eq!(path.segments[1], PathSegment::Named("b".to_owned()));
        assert_eq!(path.segments[2], PathSegment::Named("c".to_owned()));

        let path = Path::from_str("/a/b/c")?;
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0], PathSegment::Named("a".to_owned()));
        assert_eq!(path.segments[1], PathSegment::Named("b".to_owned()));
        assert_eq!(path.segments[2], PathSegment::Named("c".to_owned()));

        let path = Path::try_from_iter(vec![".", "..", "a"])?;
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0], PathSegment::CurrentDir);
        assert_eq!(path.segments[1], PathSegment::ParentDir);
        assert_eq!(path.segments[2], PathSegment::Named("a".to_owned()));

        Ok(())
    }

    #[test]
    fn test_path_canonicalize() -> anyhow::Result<()> {
        let path = Path::try_from_iter(vec!["the", "quick", "brown", "fox"])?;
        assert_eq!(path.canonicalize()?, path);

        let path = Path::try_from_iter(vec!["the", "quick", "..", "..", "brown"])?;
        assert_eq!(path.canonicalize()?, Path::try_from_iter(vec!["brown"])?);

        let path = Path::try_from_iter(vec!["the", ".", "quick", "..", "..", "brown"])?;
        assert_eq!(path.canonicalize()?, Path::try_from_iter(vec!["brown"])?);

        // Fails

        let path = Path::try_from_iter(vec![".", "the"])?;
        assert!(path.canonicalize().is_err());

        let path = Path::try_from_iter(vec!["..", "quick"])?;
        assert!(path.canonicalize().is_err());

        let path = Path::try_from_iter(vec!["the", "..", "..", "quick"])?;
        assert!(path.canonicalize().is_err());

        let path = Path::try_from_iter(vec!["the", "..", "quick", "..", "..", "brown"])?;
        assert!(path.canonicalize().is_err());

        Ok(())
    }

    #[test]
    fn test_path_display() -> anyhow::Result<()> {
        let path = Path::try_from_iter(vec!["0", "the", "quick", "brown", "fox"])?;
        let encoded = path.to_string();

        assert_eq!(encoded, "/0/the/quick/brown/fox");
        assert_eq!(path, Path::from_str(&encoded)?);

        Ok(())
    }

    #[test]
    fn test_path_equality() -> anyhow::Result<()> {
        let base_path = Path::from_str("/0/the/quick/brown/fox")?;

        assert_eq!(base_path, Path::from_str("/0/the/quick/brown/fox")?);
        assert_eq!(base_path, Path::from_str("/0/THE/QUICK/BROWN/FOX")?);
        assert_eq!(base_path, Path::from_str("/0/The/Quick/Brown/Fox")?);

        Ok(())
    }
}
