use std::convert::TryInto;

use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;
use zeroutils_ucan::UcanAuth;

use crate::filesystem::{
    DescriptorFlags, DirHandle, Entity, EntityHandle, FsError, FsResult, OpenFlags, Path,
    PermissionError,
};

use super::TraceResult;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, T> DirHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Opens the file, directory at the given path.
    pub async fn open_at<'a, U, K>(
        &self,
        path: impl TryInto<Path, Error: Into<FsError>>,
        open_flags: OpenFlags,
        descriptor_flags: DescriptorFlags,
        _ucan: UcanAuth<'a, U, K>,
    ) -> FsResult<EntityHandle<S, T>>
    where
        S: Send + Sync,
        T: Send + Sync,
        U: IpldStore,
        K: GetPublicKey,
    {
        let path = path.try_into().map_err(Into::into)?;

        // There should be at least READ flag set on the descriptor flags.
        if !descriptor_flags.contains(DescriptorFlags::READ) {
            return Err(FsError::NeedAtLeastReadFlag(path, descriptor_flags));
        }

        // Check if there is permission to read directory.
        if !self.flags.contains(DescriptorFlags::READ) {
            return Err(PermissionError::NotAllowedToReadDir.into());
        }

        // Check for descriptor flag permission escalation.
        if !self.flags.contains(DescriptorFlags::MUTATE_DIR)
            && (descriptor_flags.contains(DescriptorFlags::MUTATE_DIR)
                || descriptor_flags.contains(DescriptorFlags::WRITE)
                || open_flags.contains(OpenFlags::CREATE)
                || open_flags.contains(OpenFlags::TRUNCATE))
        {
            return Err(PermissionError::ChildPermissionEscalation(
                path,
                self.flags,
                descriptor_flags,
                open_flags,
            )
            .into());
        }

        // Handle conflicting open flags like DIRECTORY and CREATE.
        if open_flags.contains(OpenFlags::DIRECTORY)
            && (open_flags.contains(OpenFlags::CREATE)
                || open_flags.contains(OpenFlags::EXCLUSIVE)
                || open_flags.contains(OpenFlags::TRUNCATE))
        {
            return Err(FsError::InvalidOpenFlagsCombination(path, open_flags));
        }

        // TODO: Check if user has capabilities to create a file in this directory.

        // Get the entity and path directories.
        let (entity, name, pathdirs) = if open_flags.contains(OpenFlags::CREATE) {
            self.get_or_create_entity(&path, true).await?
        } else {
            match self.trace_entity(&path).await {
                Ok(TraceResult::Found {
                    entity,
                    name,
                    pathdirs,
                }) => {
                    if open_flags.contains(OpenFlags::EXCLUSIVE) {
                        return Err(FsError::OpenFlagsExclusiveButEntityExists(path, open_flags));
                    }

                    (entity, name, pathdirs)
                }
                Ok(TraceResult::NotADir { depth, .. }) => {
                    return Err(FsError::NotADirectory(Some(path.slice(..depth).to_owned())));
                }
                Ok(TraceResult::Incomplete { depth, .. }) => {
                    return Err(FsError::NotFound(path.slice(..depth).to_owned()));
                }
                Err(e) => return Err(e),
            }
        };

        // Convert the entity to an entity handle.
        let handle = match entity {
            Entity::Dir(dir) => {
                EntityHandle::from_dir(dir, name, descriptor_flags, self.root.clone(), pathdirs)
            }
            Entity::File(mut file) => {
                if open_flags.contains(OpenFlags::DIRECTORY) {
                    return Err(FsError::OpenFlagsDirectoryButEntityNotADir(
                        path, open_flags,
                    ));
                }

                if open_flags.contains(OpenFlags::TRUNCATE) {
                    file.truncate();
                }

                EntityHandle::from_file(file, name, descriptor_flags, self.root.clone(), pathdirs)
            }

            _ => return Err(FsError::NotAFileOrDir(Some(path))),
        };

        Ok(handle)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use zeroutils_key::{Ed25519KeyPair, KeyPairGenerate};
    use zeroutils_store::{MemoryStore, PlaceholderStore};

    use crate::{filesystem::RootDir, utils::fixture};

    use super::*;

    #[test_log::test(tokio::test)]
    async fn test_open_at_create() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let iss_key = Ed25519KeyPair::generate(&mut rand::thread_rng())?;
        let auth = fixture::mock_ucan_auth(&iss_key, PlaceholderStore)?;
        let root_dir = RootDir::new(store.clone());

        // Creating a non-existent file with CREATE flag.

        let dir_handle = root_dir.make_handle(DescriptorFlags::READ | DescriptorFlags::MUTATE_DIR);
        let entity_handle = dir_handle
            .open_at(
                "public/file",
                OpenFlags::CREATE | OpenFlags::EXCLUSIVE,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
                auth,
            )
            .await?;

        tracing::debug!("entity_handle: {:#?}", entity_handle);

        assert_eq!(entity_handle.name(), Some(&"file".parse()?));
        assert_eq!(entity_handle.pathdirs().len(), 1);

        // TODO:
        // Opening existing file with CREATE flag.
        // Opening existing directory with DIRECTORY flag.

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_open_at_fails() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let iss_key = Ed25519KeyPair::generate(&mut rand::thread_rng())?;
        let root_dir = RootDir::new(store.clone());

        // Creating a file in a directory with no MUTATE_DIR flag should fail.

        let dir_handle = root_dir.make_handle(DescriptorFlags::READ);
        let result = dir_handle
            .open_at(
                "public/file",
                OpenFlags::CREATE | OpenFlags::EXCLUSIVE,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
                fixture::mock_ucan_auth(&iss_key, PlaceholderStore)?,
            )
            .await;

        assert!(matches!(
            result,
            Err(FsError::PermissionError(
                PermissionError::ChildPermissionEscalation(..)
            ))
        ));

        // Opening a directory with no READ flag should fail.

        let dir_handle = root_dir.make_handle(DescriptorFlags::MUTATE_DIR);
        let result = dir_handle
            .open_at(
                "public/file",
                OpenFlags::CREATE | OpenFlags::EXCLUSIVE,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
                fixture::mock_ucan_auth(&iss_key, PlaceholderStore)?,
            )
            .await;

        assert!(matches!(
            result,
            Err(FsError::PermissionError(
                PermissionError::NotAllowedToReadDir
            ))
        ));

        // Opening a non-existent file without CREATE flag should fail.

        let dir_handle = root_dir.make_handle(DescriptorFlags::READ | DescriptorFlags::MUTATE_DIR);
        let result = dir_handle
            .open_at(
                "public/file",
                OpenFlags::EXCLUSIVE,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
                fixture::mock_ucan_auth(&iss_key, PlaceholderStore)?,
            )
            .await;

        assert!(matches!(result, Err(FsError::NotFound(..))));

        // Opening a directory with CREATE and DIRECTORY flag should fail.

        let dir_handle = root_dir.make_handle(DescriptorFlags::READ | DescriptorFlags::MUTATE_DIR);
        let result = dir_handle
            .open_at(
                "public/file",
                OpenFlags::CREATE | OpenFlags::DIRECTORY,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
                fixture::mock_ucan_auth(&iss_key, PlaceholderStore)?,
            )
            .await;

        assert!(matches!(
            result,
            Err(FsError::InvalidOpenFlagsCombination(..))
        ));

        // TODO:
        // Opening an existing file with DIRECTORY flag should fail.

        Ok(())
    }
}
