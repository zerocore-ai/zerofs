use bitflags::bitflags;
use serde::{Deserialize, Serialize};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

bitflags! {
    /// Flags to determine the capabilities of an entity.
    ///
    /// This corresponds to `descriptor-rights` in the WASI. `zerofs` does not support all the rights
    /// that WASI supports.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EntityFlags: u8 {
        /// The specifies that the file system entity can be read from.
        ///
        /// This applies to both files and directories.
        const READ = 0b0000_0001;

        /// This can only be used with files and it means that the file can be written to.
        const WRITE = 0b0000_0010;

        /// This can only be used with directories and it means that the directory and its contents
        /// can be modified.
        const MUTATE_DIR = 0b0000_0100;
    }

    /// Flags to determine how to open a path.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PathFlags: u8 {
        /// Follow symlinks.
        const SYMLINK_FOLLOW = 0b0000_0001;
    }

    /// Flags to determine how to open a file.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OpenFlags: u8 {
        /// Create the entity if it does not exist.
        const CREATE = 0b0000_0001;

        /// Fail if entity is not a directory.
        const DIRECTORY = 0b0000_0010;

        /// Fail if the entity already exists.
        const EXCLUSIVE = 0b0000_0100;

        /// Truncate the file to zero size if it exists.
        const TRUNCATE = 0b0000_1000;
    }
}
