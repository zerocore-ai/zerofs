use async_trait::async_trait;
use bytes::Bytes;
use zeroutils_store::IpldStore;
use zeroutils_wasi::io::{InputStream, StreamError, Subscribe};

use crate::filesystem::FileHandle;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file input stream.
pub struct FileInputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    _file: FileHandle<S, T>,
    _cursor: u64,
}

/// A file output stream.
pub struct FileOutputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    _file: FileHandle<S, T>,
    _cursor: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, T> FileInputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Creates a new file input stream from a file handle and an offset.
    pub fn new(file: FileHandle<S, T>, offset: u64) -> Self {
        Self {
            _file: file,
            _cursor: offset,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait]
impl<S, T> Subscribe for FileInputStream<S, T>
where
    S: IpldStore + Send + Sync + 'static,
    T: IpldStore + Send + Sync + 'static,
{
    async fn block(&self) {
        // TODO: Implement
        todo!()
    }
}

impl<S, T> InputStream for FileInputStream<S, T>
where
    S: IpldStore + Send + Sync + 'static,
    T: IpldStore + Send + Sync + 'static,
{
    fn read(&mut self, _len: u64) -> Result<Bytes, StreamError> {
        // let mut buf = Bytes::new();
        // self.file.read(self.offset, len, &mut buf).map_err(StreamError::custom)?;
        // Ok(buf)
        todo!()
    }

    /// Same as `read` except the bytes get skipped and the number of bytes skipped is returned.
    fn skip(&mut self, _len: u64) -> Result<u64, StreamError> {
        todo!()
    }
}
