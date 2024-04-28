use std::path::Path;
use std::{fs, io};

use crate::error;

#[derive(Debug)]
pub struct SymLink<'a> {
    pub from: &'a Path,
    pub to: &'a Path,
}

impl<'a> SymLink<'a> {
    fn new(from: &'a Path, to: &'a Path) -> Self {
        SymLink { from, to }
    }
}

// TODO: add this to the constructor of the symlink
pub fn symlink<'a>(from: &'a Path, to: &'a Path) -> io::Result<SymLink<'a>> {
    let current_dir = std::env::current_dir()?;

    let from_absolute = current_dir.join(from);
    let to_absolute = current_dir.join(to);

    std::os::unix::fs::symlink(from_absolute, to_absolute)?;

    Ok(SymLink::new(from, to))
}

pub fn is_symlink(path: &Path) -> io::Result<bool> {
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();

    Ok(file_type.is_symlink())
}

pub fn exists(path: &Path) -> bool {
    path.try_exists().is_ok_and(|exists| exists)
}

pub fn remove(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}

pub fn create_parent_path(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path.parent().ok_or_else(error::invalid_path)?)
}

pub fn read(from: &Path) -> io::Result<String> {
    fs::read_to_string(from)
}

pub fn write(to: &Path, buffer: &str) -> io::Result<()> {
    fs::write(to, buffer)
}
