use std::{
    collections::{hash_map, BTreeMap, HashMap},
    convert::TryInto,
    fmt::{self, Debug},
    sync::{Arc, RwLock, RwLockReadGuard},
};

use aliasable::boxed::AliasableBox;
use serde::{
    de::{self, DeserializeSeed},
    Deserialize, Deserializer, Serialize, Serializer,
};
use zeroutils_store::{
    ipld::cid::Cid, IpldReferences, IpldStore, Storable, StoreError, StoreResult,
};
use zeroutils_ucan::SignedUcan;

use super::{
    CidLink, DirDescriptor, Entity, EntityFlags, EntityType, File, FileDescriptor, FsError,
    FsResult, Link, Metadata, OpenFlags, Path, PathFlags, PathSegment,
};

//--------------------------------------------------------------------------------------------------
// Types: Dir
//--------------------------------------------------------------------------------------------------

/// Represents a directory in the `zerofs` file system.
///
/// Since zerofs is a capability-based file system, a `UCAN` needs to provided that lets the file
/// system grant access to the directory's contents.
#[derive(Clone)]
pub struct Dir<S>
where
    S: IpldStore,
{
    inner: Arc<DirInner<S>>,
}

struct DirInner<S>
where
    S: IpldStore,
{
    /// Directory metadata.
    pub(crate) metadata: Metadata,

    /// The store used to persist blocks in the directory.
    pub(crate) store: S,

    /// The entries in the directory.
    pub(crate) entries: RwLock<HashMap<String, CidLink<S>>>,
}

//--------------------------------------------------------------------------------------------------
// Types: *
//--------------------------------------------------------------------------------------------------

/// Iterator over the entries in a directory.
///
/// ## Important
///
/// This is a self-referential struct that holds a reference to the read guard of the entries in the
/// directory.
pub struct EntriesGuardIter<'a, S>
where
    S: IpldStore,
{
    /// Holds a reference to the array's data.
    /// Declared first to ensure it is dropped before the guard.
    inner: hash_map::Iter<'a, String, Link<Cid, S>>,

    /// Must not move this box as it is aliased by `inner`.
    #[allow(dead_code)]
    guard: AliasableBox<RwLockReadGuard<'a, HashMap<String, CidLink<S>>>>,
}

enum FindResult<S>
where
    S: IpldStore,
{
    /// The entity was found.
    Found(Dir<S>),

    /// Intermediate path is not a directory.
    NotADir { dir: Dir<S>, depth: usize },

    /// The entity was not found.
    Incomplete { dir: Dir<S>, depth: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DirSerializable {
    metadata: Metadata,
    entries: BTreeMap<String, Cid>,
}

pub(crate) struct DirDeserializeSeed<S> {
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods: Iter
//--------------------------------------------------------------------------------------------------

impl<'a, S> EntriesGuardIter<'a, S>
where
    S: IpldStore,
{
    fn from(dir: &'a Dir<S>) -> Self {
        let guard = AliasableBox::from_unique(Box::new(dir.inner.entries.read().unwrap()));
        let inner = guard.iter();
        let inner = unsafe {
            std::mem::transmute::<
                hash_map::Iter<String, CidLink<S>>,
                hash_map::Iter<String, CidLink<S>>,
            >(inner)
        };
        EntriesGuardIter { inner, guard }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: Dir
//--------------------------------------------------------------------------------------------------

impl<S> Dir<S>
where
    S: IpldStore,
{
    /// Creates a new directory with the given store.
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(DirInner {
                metadata: Metadata::new(EntityType::Dir),
                store,
                entries: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Creates a new directory descriptor for the directory.
    pub fn into_fd(self, entity_flags: EntityFlags) -> DirDescriptor<S> {
        DirDescriptor {
            entity: self,
            flags: entity_flags,
        }
    }

    /// Deserializes to a `Dir` using an arbitrary deserializer and store.
    pub fn deserialize_with<'de>(
        deserializer: impl Deserializer<'de, Error: Into<FsError>>,
        store: S,
    ) -> FsResult<Self> {
        DirDeserializeSeed::new(store)
            .deserialize(deserializer)
            .map_err(Into::into)
    }

    /// Tries to create a new `Dir` from a serializable representation.
    pub(crate) fn try_from_serializable(serializable: DirSerializable, store: S) -> FsResult<Self> {
        let entries: HashMap<_, _> = serializable
            .entries
            .into_iter()
            .map(|(k, v)| (k, Link::from(v)))
            .collect();

        Ok(Dir {
            inner: Arc::new(DirInner {
                metadata: serializable.metadata,
                store,
                entries: RwLock::new(entries),
            }),
        })
    }

    /// Returns an iterator over the entries in the directory.
    pub fn entries(&self) -> EntriesGuardIter<'_, S> {
        EntriesGuardIter::from(self)
    }

    /// Adds the given entries to the directory.
    pub fn add_entries(&self, entries: impl IntoIterator<Item = (String, Cid)>) {
        self.inner
            .entries
            .write()
            .unwrap()
            .extend(entries.into_iter().map(|(k, v)| (k, CidLink::from(v))));
    }

    /// Returns the metadata for the directory.
    pub fn metadata(&self) -> &Metadata {
        &self.inner.metadata
    }

    /// Gets the entity with the given name from the directory.
    async fn get_entity(&self, path_segment: &PathSegment) -> FsResult<Option<&Entity<S>>> {
        if !path_segment.is_named() {
            return Ok(None);
        }

        if let Some((_, link)) = self
            .entries()
            .find(|(name, _)| *name == &path_segment.to_string())
        {
            let entity = link.resolve_entity(self.inner.store.clone()).await?;
            return Ok(Some(entity));
        }

        Ok(None)
    }

    /// Gets the leaf directory at the given path.
    async fn get_leaf_dir(&self, path: &Path) -> FsResult<FindResult<S>> {
        let canonical_path = path.canonicalize()?;
        let mut dir = self;
        for (depth, segment) in canonical_path.segments().iter().enumerate() {
            match dir.get_entity(segment).await? {
                Some(Entity::Dir(d)) => dir = d,
                // TODO: Some(Entity::Symlink(s)) => { ... } // To follow Symlink or not.
                Some(_) => {
                    return Ok(FindResult::NotADir {
                        dir: dir.clone(),
                        depth,
                    })
                }
                _ => {
                    return Ok(FindResult::Incomplete {
                        dir: dir.clone(),
                        depth,
                    })
                }
            }
        }

        Ok(FindResult::Found(dir.clone()))
    }

    /// Gets the leaf directory at the given path, creating it if it does not exist.
    async fn get_or_create_leaf_dir(&self, path: &Path) -> FsResult<Dir<S>> {
        match self.get_leaf_dir(path).await? {
            FindResult::Incomplete {
                dir: start_head,
                depth,
            } => {
                let mut end_head = start_head.clone();
                let mut child: Option<Cid> = None;
                for (i, segment) in path
                    .segments()
                    .iter()
                    .rev()
                    .take(path.len() - depth)
                    .enumerate()
                {
                    let dir = Dir::new(start_head.inner.store.clone());
                    if let Some(cid) = child {
                        dir.add_entries([(segment.to_string(), cid)]);
                    }

                    // Persist the directory to the store.
                    let cid = dir.store().await?;
                    child = Some(cid);

                    if i == 0 {
                        end_head = dir;
                    }
                }

                // Update the head directory with the new child.
                if let Some(cid) = child {
                    start_head.add_entries([(path.segments().last().unwrap().to_string(), cid)]);
                }

                Ok(end_head)
            }
            FindResult::Found(dir) => Ok(dir),
            FindResult::NotADir { depth, .. } => {
                let path = Path::try_from_iter(path.iter().take(depth).cloned())?;
                Err(FsError::NotADirectory(Some(path)))
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: DirDescriptor
//--------------------------------------------------------------------------------------------------

impl<S> DirDescriptor<S>
where
    S: IpldStore,
{
    /// Opens the file, directory at the given path.
    pub async fn open_at<'a, T>(
        &self,
        path: impl TryInto<Path, Error: Into<FsError>>,
        path_flags: impl Into<PathFlags>,
        open_flags: impl Into<OpenFlags>,
        entity_flags: impl Into<EntityFlags>,
        _ucan: SignedUcan<'a, T>,
    ) -> FsResult<FileDescriptor<S>>
    where
        T: IpldStore,
    {
        let _path_flags = path_flags.into();
        let open_flags = open_flags.into();
        let entity_flags = entity_flags.into();

        // TODO: Do important checks here.
        // TODO: Handle the UCAN.

        let path = path.try_into().map_err(Into::into)?;
        let (init, last) = path.split_last();
        let init = Path::try_from_iter(init.iter().cloned())?;

        let dir = if open_flags.contains(OpenFlags::CREATE) {
            self.get_or_create_leaf_dir(&init).await?
        } else {
            match self.get_leaf_dir(&init).await? {
                FindResult::Found(dir) => dir,
                FindResult::Incomplete { depth, .. } => {
                    let path = Path::try_from_iter(init.iter().take(depth).cloned())?;
                    return Err(FsError::NotFound(path));
                }
                FindResult::NotADir { depth, .. } => {
                    let path = Path::try_from_iter(init.iter().take(depth).cloned())?;
                    return Err(FsError::NotADirectory(Some(path)));
                }
            }
        };

        let fd = match dir.get_entity(last).await? {
            Some(entity) => match entity {
                Entity::File(f) => f.clone().into_fd(entity_flags),
                _ => return Err(FsError::NotAFile(Some(path))),
            },
            None => {
                if !open_flags.contains(OpenFlags::CREATE) {
                    return Err(FsError::NotFound(init)); // TODO: Join init and last.
                }

                let file = File::new(dir.inner.store.clone());
                let cid = file.store().await?;
                dir.add_entries([(last.to_string(), cid)]);

                file.into_fd(entity_flags)
            }
        };

        Ok(fd)
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: DirDeserializeSeed
//--------------------------------------------------------------------------------------------------

impl<S> DirDeserializeSeed<S> {
    fn new(store: S) -> Self {
        Self { store }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> IpldReferences for Dir<S>
where
    S: IpldStore,
{
    fn references<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Cid> + 'a> {
        Box::new(self.entries().map(|(_, v)| v.cid()))
    }
}

impl<S> Storable<S> for Dir<S>
where
    S: IpldStore,
{
    async fn store(&self) -> StoreResult<Cid> {
        self.inner.store.put(self).await
    }

    async fn load(cid: &Cid, store: S) -> StoreResult<Self> {
        let serializable: DirSerializable = store.get(cid).await?;
        Dir::try_from_serializable(serializable, store).map_err(StoreError::custom)
    }
}

impl<S> Debug for Dir<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir")
            .field("metadata", &self.inner.metadata)
            .field(
                "entries",
                &self.entries().map(|(_, v)| v.cid()).collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl<S> Serialize for Dir<S>
where
    S: IpldStore,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let serializable = DirSerializable {
            metadata: self.inner.metadata.clone(),
            entries: self.entries().map(|(k, v)| (k.clone(), *v.cid())).collect(),
        };

        serializable.serialize(serializer)
    }
}

impl<'de, S> DeserializeSeed<'de> for DirDeserializeSeed<S>
where
    S: IpldStore,
{
    type Value = Dir<S>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable = DirSerializable::deserialize(deserializer)?;
        Dir::try_from_serializable(serializable, self.store).map_err(de::Error::custom)
    }
}

impl<S> PartialEq for Dir<S>
where
    S: IpldStore,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<S> PartialEq for DirInner<S>
where
    S: IpldStore,
{
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.entries.read().unwrap().len() == other.entries.read().unwrap().len()
            && *self.entries.read().unwrap() == *other.entries.read().unwrap()
    }
}

impl<S> Debug for FindResult<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FindResult::Found(dir) => f.debug_tuple("Found").field(dir).finish(),
            FindResult::NotADir { dir, depth } => f
                .debug_struct("NotADir")
                .field("dir", dir)
                .field("depth", depth)
                .finish(),
            FindResult::Incomplete { dir, depth } => f
                .debug_struct("Incomplete")
                .field("dir", dir)
                .field("depth", depth)
                .finish(),
        }
    }
}

impl<'a, S> Iterator for EntriesGuardIter<'a, S>
where
    S: IpldStore,
{
    type Item = (&'a String, &'a Link<Cid, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Ok;
    use zeroutils_key::{Ed25519KeyPair, KeyPairGenerate};
    use zeroutils_store::MemoryStore;

    use crate::utils::fixture::mock_ucan;

    use super::*;

    #[tokio::test]
    async fn test_dir_constructor() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let dir = Dir::new(store);

        assert!(dir.inner.entries.read().unwrap().is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_dir_add_entries() -> anyhow::Result<()> {
        let store = MemoryStore::default();

        let dir = Dir::new(store);
        dir.add_entries([
            (
                "file1".to_string(),
                Cid::from_str("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq")?,
            ),
            (
                "file2".to_string(),
                Cid::from_str("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq")?,
            ),
        ]);

        assert_eq!(dir.inner.entries.read().unwrap().len(), 2);
        assert_eq!(
            dir.inner
                .entries
                .read()
                .unwrap()
                .get("file1")
                .unwrap()
                .cid(),
            &Cid::from_str("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq")?
        );
        assert_eq!(
            dir.inner
                .entries
                .read()
                .unwrap()
                .get("file2")
                .unwrap()
                .cid(),
            &Cid::from_str("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq")?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_dir_stores_loads() -> anyhow::Result<()> {
        let store = MemoryStore::default();

        let dir = Dir::new(store.clone());
        dir.add_entries([(
            "file1".to_string(),
            Cid::from_str("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq")?,
        )]);

        let cid = dir.store().await?;
        let loaded_dir = Dir::load(&cid, store.clone()).await?;

        assert_eq!(dir, loaded_dir);

        Ok(())
    }

    #[tokio::test]
    async fn test_dir_open_at() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let issuer_key = Ed25519KeyPair::generate(&mut rand::thread_rng())?;
        let ucan = mock_ucan(&issuer_key, store.clone())?;

        let dir = Dir::new(store.clone()).into_fd(EntityFlags::MUTATE_DIR);
        let fd = dir
            .open_at(
                "public/file",
                PathFlags::SYMLINK_FOLLOW,
                OpenFlags::CREATE,
                EntityFlags::WRITE,
                ucan,
            )
            .await?;

        store.print().await;
        println!("entity: {:#?}", fd); // TODO: Remove

        Ok(())
    }
}
