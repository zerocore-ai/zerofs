#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

trait Store {
    fn write_block(&self, block_id: u64, data: Vec<u8>) -> Result<(), String>;
    fn read_block(&self, block_id: u64) -> Result<Vec<u8>, String>;
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct InMemoryStore {
    blocks: Mutex<HashMap<u64, Vec<u8>>>,
}

enum FileType {
    File {
        content: Vec<u8>,
        cursor: usize,
        metadata: Metadata,
    },
    Directory {
        entries: HashMap<String, Arc<Mutex<Metadata>>>,
        metadata: Metadata,
    },
}

struct FileSystem {
    root: Arc<Mutex<FileType>>,
}

struct Metadata {
    size: usize,
    created_at: SystemTime,
    modified_at: SystemTime,
    entity_type: EntityType,
}

enum EntityType {
    File,
    Directory,
}

struct FileDescriptor {
    file: Arc<Mutex<FileType>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: Arc::new(Mutex::new(FileType::new_dir())),
        }
    }

    fn mkdir(&self, path: &str) -> Result<(), String> {
        let mut root = self.root.lock().unwrap();
        match &mut *root {
            FileType::Directory { entries, metadata } => {
                entries.insert(
                    path.to_string(),
                    Arc::new(Mutex::new(Metadata::new(FileType::new_dir()))),
                );
                metadata.update_modified();
                Ok(())
            }
            _ => Err("Root is not a directory".to_string()),
        }
    }

    fn rmdir(&self, path: &str) -> Result<(), String> {
        let mut root = self.root.lock().unwrap();
        match &mut *root {
            FileType::Directory { entries, .. } => {
                if let Some(Arc { .. }) = entries.get(path) {
                    if let Ok(dir) = entries[path].lock() {
                        match &*dir {
                            Metadata {
                                entity_type: EntityType::Directory,
                                ..
                            } => {
                                if dir_entries.is_empty() {
                                    entries.remove(path);
                                    return Ok(());
                                } else {
                                    return Err("Directory is not empty".to_string());
                                }
                            }
                            _ => return Err("Path is not a directory".to_string()),
                        }
                    }
                }
                Err("Directory does not exist".to_string())
            }
            _ => Err("Root is not a directory".to_string()),
        }
    }

    fn rename(&self, old_path: &str, new_path: &str) -> Result<(), String> {
        let mut root = self.root.lock().unwrap();
        match &mut *root {
            FileType::Directory { entries, .. } => {
                let entry = entries
                    .remove(old_path)
                    .ok_or_else(|| "Old path does not exist".to_string())?;
                entries.insert(new_path.to_string(), entry);
                Ok(())
            }
            _ => Err("Root is not a directory".to_string()),
        }
    }
}

impl FileDescriptor {
    fn read(&mut self, buffer: &mut Vec<u8>, size: usize) -> Result<usize, String> {
        let mut file = self.file.lock().unwrap();
        match &mut *file {
            FileType::File {
                content, cursor, ..
            } => {
                let end = (*cursor + size).min(content.len());
                buffer.extend_from_slice(&content[*cursor..end]);
                let read_bytes = end - *cursor;
                *cursor = end; // Update the cursor position
                Ok(read_bytes)
            }
            _ => Err("Not a file".to_string()),
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), String> {
        let mut file = self.file.lock().unwrap();
        match &mut *file {
            FileType::File {
                content,
                cursor,
                metadata,
            } => {
                content.splice(*cursor..*cursor, data.iter().cloned());
                *cursor += data.len();
                metadata.size = content.len();
                metadata.update_modified();
                Ok(())
            }
            _ => Err("Not a file".to_string()),
        }
    }

    // Methods like open, close, lseek, and stat would need additional metadata tracking and logic
}

impl FileType {
    fn new_file() -> Self {
        unimplemented!()
    }

    fn new_dir() -> Self {
        unimplemented!()
    }
}

impl Metadata {
    fn new(entity_type: EntityType) -> Self {
        let now = SystemTime::now();
        Metadata {
            size: 0, // Initial size, will be updated for files
            created_at: now,
            modified_at: now,
            entity_type,
        }
    }

    fn update_modified(&mut self) {
        self.modified_at = SystemTime::now();
    }
}

impl InMemoryStore {
    fn new() -> Self {
        InMemoryStore {
            blocks: Mutex::new(HashMap::new()),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Store for InMemoryStore {
    fn write_block(&self, block_id: u64, data: Vec<u8>) -> Result<(), String> {
        let mut blocks = self.blocks.lock().unwrap();
        blocks.insert(block_id, data);
        Ok(())
    }

    fn read_block(&self, block_id: u64) -> Result<Vec<u8>, String> {
        let blocks = self.blocks.lock().unwrap();
        blocks
            .get(&block_id)
            .cloned()
            .ok_or_else(|| "Block not found".to_string())
    }
}
