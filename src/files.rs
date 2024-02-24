use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SymLink {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl SymLink {
    fn new(from: &Path, to: &Path) -> Self {
        SymLink {
            from: PathBuf::from(from),
            to: PathBuf::from(to),
        }
    }
}

pub fn symlink(from: &Path, to: &Path) -> io::Result<SymLink> {
    let current_dir = std::env::current_dir()?;

    let from_absolute = current_dir.join(from);
    let to_absolute = current_dir.join(to);

    std::os::unix::fs::symlink(from_absolute, to_absolute)?;

    Ok(SymLink::new(from, to))
}

pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::rename(from, to)
}

pub fn read(from: &Path) -> io::Result<String> {
    std::fs::read_to_string(from)
}

pub fn write(to: &Path, buffer: &str) -> io::Result<()> {
    std::fs::write(to, buffer)
}
