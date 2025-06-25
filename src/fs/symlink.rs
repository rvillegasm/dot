use std::fs;
use std::path::{Path, PathBuf};

use crate::error::DotError;
use crate::fs::FileSystem;

/// Represents a symbolic link between two paths.
#[derive(Debug, Clone)]
pub struct SymLink {
    /// The source path (the actual file/directory)
    pub from: PathBuf,
    /// The destination path (the symlink)
    pub to: PathBuf,
}

impl SymLink {
    /// Create a new SymLink instance
    pub fn new<F: Into<PathBuf>, T: Into<PathBuf>>(from: F, to: T) -> Self {
        SymLink {
            from: from.into(),
            to: to.into(),
        }
    }
}

/// A trait for handling symbolic link operations
pub trait SymLinkOperations {
    /// Create a symlink from a source to a destination
    fn create_symlink<F: AsRef<Path>, T: AsRef<Path>>(
        &self,
        from: F,
        to: T,
    ) -> Result<SymLink, DotError>;

    /// Check if a path is a symlink
    fn is_symlink<P: AsRef<Path>>(&self, path: P) -> Result<bool, DotError>;
}

/// Implementation of SymLinkOperations for Unix-like systems
#[derive(Clone)]
pub struct UnixSymLinkOperations<F: FileSystem> {
    fs: F,
}

impl<F: FileSystem> UnixSymLinkOperations<F> {
    pub fn new(fs: F) -> Self {
        Self { fs }
    }
}

impl<F: FileSystem> SymLinkOperations for UnixSymLinkOperations<F> {
    fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to: Q,
    ) -> Result<SymLink, DotError> {
        let current_dir = self.fs.current_dir()?;
        let from_abs = current_dir.join(from.as_ref());
        let to_abs = current_dir.join(to.as_ref());
        std::os::unix::fs::symlink(&from_abs, &to_abs)?;
        Ok(SymLink::new(
            from.as_ref().to_path_buf(),
            to.as_ref().to_path_buf(),
        ))
    }

    fn is_symlink<P: AsRef<Path>>(&self, path: P) -> Result<bool, DotError> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        Ok(file_type.is_symlink())
    }
}
