use async_trait::async_trait;
use bytes::Bytes;
use zeroutils_store::IpldStore;
use zeroutils_wasi::io::{InputStream, StreamError, Subscribe};

use super::FileDescriptor;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file input stream.
pub struct FileInputStream<S>
where
    S: IpldStore,
{
    _file: FileDescriptor<S>,
    _cursor: u64,
}

/// A file output stream.
pub struct FileOutputStream<S>
where
    S: IpldStore,
{
    _file: FileDescriptor<S>,
    _cursor: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> FileInputStream<S>
where
    S: IpldStore,
{
    /// Creates a new file input stream from a file descriptor and an offset.
    pub fn new(file: FileDescriptor<S>, offset: u64) -> Self {
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
impl<S> Subscribe for FileInputStream<S>
where
    S: IpldStore + Sync + Send + 'static,
{
    async fn block(&self) {
        todo!()
    }
}

impl<S> InputStream for FileInputStream<S>
where
    S: IpldStore + Sync + Send + 'static,
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
