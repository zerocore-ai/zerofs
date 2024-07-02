use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;
use zeroutils_ucan::UcanAuth;

use crate::filesystem::{FileHandle, FileInputStream, FileOutputStream, FsResult};

//--------------------------------------------------------------------------------------------------
// Methods: FileHandle
//--------------------------------------------------------------------------------------------------

impl<S, T> FileHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Returns a stream to write to the file.
    pub fn write_via_stream<U, K>(
        &self,
        _offset: u64,
        _ucan: UcanAuth<U, K>,
    ) -> FsResult<FileOutputStream<S, T>>
    where
        U: IpldStore,
        K: GetPublicKey,
    {
        todo!()
    }
}
