use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SymLink {
    from: PathBuf,
    to: PathBuf,
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
    std::os::unix::fs::symlink(from, to)?;

    Ok(SymLink::new(from, to))
}

pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::rename(from, to)
}
