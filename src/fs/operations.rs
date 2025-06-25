use std::fs;
use std::path::{Path, PathBuf};

use crate::error::DotError;
use crate::fs::FileSystem;

/// Standard implementation of file system operations using `std::fs`.
#[derive(Default, Debug, Clone)]
pub struct StdFileSystem;

impl FileSystem for StdFileSystem {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().try_exists().is_ok_and(|exists| exists)
    }

    fn read<P: AsRef<Path>>(&self, path: P) -> Result<String, DotError> {
        fs::read_to_string(path).map_err(DotError::Io)
    }

    fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), DotError> {
        fs::write(path, content).map_err(DotError::Io)
    }

    fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), DotError> {
        if path.as_ref().is_dir() {
            fs::remove_dir_all(path).map_err(DotError::Io)
        } else {
            fs::remove_file(path).map_err(DotError::Io)
        }
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), DotError> {
        fs::rename(from, to).map_err(DotError::Io)
    }

    fn create_parent_path<P: AsRef<Path>>(&self, path: P) -> Result<(), DotError> {
        let parent = path.as_ref().parent().ok_or(DotError::InvalidPath)?;
        fs::create_dir_all(parent).map_err(|_| DotError::InvalidPath)
    }

    fn current_dir(&self) -> Result<PathBuf, DotError> {
        std::env::current_dir().map_err(DotError::Io)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), DotError> {
        std::fs::create_dir_all(path).map_err(DotError::from)
    }
}
