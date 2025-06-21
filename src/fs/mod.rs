use std::path::{Path, PathBuf};

use crate::error::DotError;

pub mod operations;
pub mod symlink;

/// A trait defining file system operations.
pub trait FileSystem {
    /// Check if a path exists in the file system.
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    
    /// Read the content of a file into a string.
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<String, DotError>;
    
    /// Write a string buffer to a file.
    fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), DotError>;
    
    /// Remove a file or directory.
    fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), DotError>;
    
    /// Rename (move) a file or directory.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), DotError>;
    
    /// Create all parent directories for a given path.
    fn create_parent_path<P: AsRef<Path>>(&self, path: P) -> Result<(), DotError>;
    
    /// Get the current working directory.
    fn current_dir(&self) -> Result<PathBuf, DotError>;
}
