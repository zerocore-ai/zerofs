use zeroutils_store::IpldStore;

use super::Metadata;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file in the file system.
#[derive(Debug)]
pub struct File<S>
where
    S: IpldStore,
{
    /// The name of the file.
    name: String,

    /// File metadata.
    metadata: Metadata,

    /// The store used to persist blocks in the file.
    store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> File<S>
where
    S: IpldStore,
{
    // /// Returns a stream for reading the file starting at `offset`.
    // pub fn read(&self, _offset: u64) -> FsResult<FileInputStream> {
    //     unimplemented!("read")
    // }

    // /// Reads `length` bytes from the file starting at `offset`.
    // ///
    // /// The second element of the tuple indicates whether the end of the file has been reached.
    // pub fn read_exact(&self, _length: u64, _offset: u64) -> FsResult<(Vec<u8>, bool)> {
    //     unimplemented!("read")
    // }
}
