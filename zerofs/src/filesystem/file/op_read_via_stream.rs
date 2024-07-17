use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;
use zeroutils_ucan::UcanAuth;

use crate::filesystem::{FileHandle, FileInputStream, FsResult};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, T> FileHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Returns a stream to read from the file.
    pub async fn read_via_stream<U, K>(
        &self,
        offset: u64,
        _ucan: UcanAuth<'_, U, K>,
    ) -> FsResult<FileInputStream<S, T>>
    where
        U: IpldStore,
        K: GetPublicKey,
    {
        // TODO: Check if user has capabilities to read the file.
        // Ok(FileInputStream::from(self.clone()).await)
        todo!()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------
