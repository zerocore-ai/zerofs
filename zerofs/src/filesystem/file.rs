//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file in the file system.
pub struct File {
    /// The name of the file.
    pub name: String,

    /// The content of the file.
    pub content: Option<Vec<u8>>,
}
