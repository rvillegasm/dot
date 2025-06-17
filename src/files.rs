use std::fs;
use std::path::{Path, PathBuf};

use crate::error::DotError;

#[derive(Debug)]
pub struct SymLink {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl SymLink {
    fn new<F: Into<PathBuf>, T: Into<PathBuf>>(from: F, to: T) -> Self {
        SymLink {
            from: from.into(),
            to: to.into(),
        }
    }
}

// TODO: add this to the constructor of the symlink
pub fn symlink<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> Result<SymLink, DotError> {
    let from_abs = std::env::current_dir()?.join(from.as_ref());
    let to_abs = std::env::current_dir()?.join(to.as_ref());
    std::os::unix::fs::symlink(&from_abs, &to_abs)?;
    Ok(SymLink::new(from_abs, to_abs))
}

pub fn is_symlink<P: AsRef<Path>>(path: P) -> Result<bool, DotError> {
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();
    Ok(file_type.is_symlink())
}

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().try_exists().is_ok_and(|exists| exists)
}

pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), DotError> {
    if path.as_ref().is_dir() {
        fs::remove_dir_all(path).map_err(DotError::Io)
    } else {
        fs::remove_file(path).map_err(DotError::Io)
    }
}

pub fn rename<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> Result<(), DotError> {
    fs::rename(from, to).map_err(DotError::Io)
}

pub fn create_parent_path<P: AsRef<Path>>(path: P) -> Result<(), DotError> {
    let parent = path.as_ref().parent().ok_or(DotError::InvalidPath)?;
    fs::create_dir_all(parent).map_err(|_| DotError::InvalidPath)
}

pub fn read<P: AsRef<Path>>(from: P) -> Result<String, DotError> {
    fs::read_to_string(from).map_err(DotError::Io)
}

pub fn write<T: AsRef<Path>>(to: T, buffer: &str) -> Result<(), DotError> {
    fs::write(to.as_ref(), buffer).map_err(DotError::Io)
}
