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
    if from.try_exists().is_ok_and(|p| p) && to.try_exists().is_ok_and(|p| !p) {
        std::os::unix::fs::symlink(from, to)?;

        Ok(SymLink::new(from, to))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "expected files not present",
        ))
    }
}
