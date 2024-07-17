use zeroutils_key::GetPublicKey;
use zeroutils_store::IpldStore;
use zeroutils_ucan::UcanAuth;

use crate::filesystem::{DescriptorFlags, FileHandle, FileOutputStream, FsResult};

//--------------------------------------------------------------------------------------------------
// Methods
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
        if !self.flags().contains(DescriptorFlags::WRITE) {
            todo!()
        }

        // TODO: Check if user has capabilities to write to the file.

        todo!()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------
