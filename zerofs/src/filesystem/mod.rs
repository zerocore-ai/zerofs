//! The file system module.

mod descriptor;
mod dir;
mod entity;
mod error;
mod file;
mod flag;
mod io;
mod kind;
mod link;
mod metadata;
mod path;
mod store;
mod symlink;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use descriptor::*;
pub use dir::*;
pub use entity::*;
pub use error::*;
pub use file::*;
pub use flag::*;
pub use io::*;
pub use kind::*;
pub use link::*;
pub use metadata::*;
pub use path::*;
pub use store::*;
pub use symlink::*;
