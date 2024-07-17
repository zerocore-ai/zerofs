use std::{mem, pin::Pin};

use aliasable::boxed::AliasableBox;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncRead, AsyncReadExt};
use zeroutils_store::IpldStore;
use zeroutils_wasi::io::{Await, InputStream, StreamError};

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
    /// Temporary buffer for recently fetched chunk or a stream error.
    buffer: Result<BytesMut, StreamError>,

    /// An async reader for the file content.
    ///
    /// ## Important
    ///
    /// Holds a reference to other fields in this struct. Declared first to ensure it is dropped
    /// before the other fields.
    reader: Pin<Box<dyn AsyncRead + Send + Sync + 'static>>,

    /// The file handle.
    ///
    /// ## Warning
    ///
    /// Field must not be moved as it is referenced by `reader`.
    handle: AliasableBox<FileHandle<S, T>>,
}

/// A file output stream.
pub struct FileOutputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    handle: FileHandle<S, T>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, T> FileInputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Creates an input stream for reading a file's content from its file handle.
    pub async fn from(handle: FileHandle<S, T>) -> Self {
        // Store the handle in the heap and make it aliasable.
        let handle = AliasableBox::from_unique(Box::new(handle));

        // If the file contains a Cid for its content, create a reader for it.
        let reader: Pin<Box<dyn AsyncRead + Send + Sync>> = match handle.get_content() {
            Some(cid) => handle.get_store().get_bytes(cid).await.unwrap(),
            None => Box::pin(&[][..]),
        };

        // Unsafe magic to escape Rust ownership grip.
        let reader: Pin<Box<dyn AsyncRead + Send + Sync + 'static>> =
            unsafe { std::mem::transmute(reader) };

        Self {
            buffer: Ok(BytesMut::new()),
            reader,
            handle,
        }
    }

    /// Takes error or bytes stored in the buffer. If the buffer contains unused bytes, it
    /// returns a slice of it of the given length or the entire bytes if it is less than the
    /// requested length.
    pub fn take_buffer(&mut self, len: u64) -> Result<BytesMut, StreamError> {
        let buffer = mem::replace(&mut self.buffer, Ok(BytesMut::new()));
        match buffer {
            Ok(mut bytes) => {
                // Split the buffer into a slice of the requested length if it is less than the
                // requested length.
                let tail = if bytes.len() > len as usize {
                    bytes.split_off(len as usize)
                } else {
                    BytesMut::new()
                };

                self.buffer = Ok(tail);

                Ok(bytes)
            }
            Err(e) => Err(e),
        }
    }
}

impl<S, T> FileOutputStream<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    // /// Creates a new file output stream from a file handle and an offset.
    // pub fn from(file: FileHandle<S, T>, offset: u64) -> Self {
    //     Self { handle: file }
    // }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait]
impl<S, T> Await for FileInputStream<S, T>
where
    S: IpldStore + Send + Sync + 'static,
    T: IpldStore + Send + Sync + 'static,
{
    async fn wait(&mut self) {
        let mut bytes = match self.handle.get_store().get_node_block_max_size() {
            Some(max_size) => BytesMut::with_capacity(max_size as usize),
            None => BytesMut::new(),
        };

        // Attempt to read the next chunk and update the buffer.
        match self.reader.read_buf(&mut bytes).await {
            Ok(_) => self.buffer = Ok(bytes),
            Err(e) => self.buffer = Err(StreamError::IoError(e)),
        };
    }
}

impl<S, T> InputStream for FileInputStream<S, T>
where
    S: IpldStore + Send + Sync + 'static,
    T: IpldStore + Send + Sync + 'static,
{
    fn read(&mut self, len: u64) -> Result<Bytes, StreamError> {
        self.take_buffer(len).map(|bytes| bytes.into())
    }

    fn skip(&mut self, len: u64) -> Result<u64, StreamError> {
        self.take_buffer(len).map(|bytes| bytes.len() as u64)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use zeroutils_store::MemoryStore;

    use crate::filesystem::{DescriptorFlags, RootDir};

    use super::*;

    #[tokio::test]
    async fn test_file_input_stream_constructor() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let data = fixtures::sample_data();
        let cid = store.put_bytes(&data[..]).await?;

        // Get root handle.
        let root = RootDir::new(store.clone());
        let root = root.make_handle(DescriptorFlags::READ | DescriptorFlags::MUTATE_DIR);

        // // Create file.
        // let file = todo!();

        Ok(())
    }
}

#[cfg(test)]
mod fixtures {
    use super::*;

    pub(super) fn sample_data() -> Bytes {
        Bytes::from(&b"Lorem ipsum dolor sit amet, consectetur adipiscing elit."[..])
    }
}
