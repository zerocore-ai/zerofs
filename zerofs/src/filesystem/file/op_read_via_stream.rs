use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;
use zeroutils_ucan::UcanAuth;

use crate::filesystem::{FileHandle, FileInputStream, FsResult};

//--------------------------------------------------------------------------------------------------
// Methods: FileHandle
//--------------------------------------------------------------------------------------------------

impl<S, T> FileHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Returns a stream to read from the file.
    pub fn read_via_stream<U, K>(
        &self,
        _offset: u64,
        _ucan: UcanAuth<U, K>,
    ) -> FsResult<FileInputStream<S, T>>
    where
        U: IpldStore,
        K: GetPublicKey,
    {
        todo!()
    }
}
