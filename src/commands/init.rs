use std::fs;
use std::path::Path;

use crate::commands::Command;
use crate::error::{Error, Result};
use crate::manifest::MANIFEST_FILE;

pub struct InitCommand;

impl InitCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for InitCommand {
    fn execute(self) -> Result<()> {
        let path = Path::new(MANIFEST_FILE);
        if path.exists() {
            return Err(Error::AlreadyExists(path.to_path_buf()));
        }
        fs::write(path, "")?;
        log::info!("Initialized dot repository");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Test helper that initializes a manifest at a specific path
    fn init_at(path: &Path) -> Result<()> {
        if path.exists() {
            return Err(Error::AlreadyExists(path.to_path_buf()));
        }
        fs::write(path, "")?;
        Ok(())
    }

    #[test]
    fn creates_manifest_file() {
        let temp = TempDir::new().unwrap();
        let manifest_path = temp.path().join(MANIFEST_FILE);

        init_at(&manifest_path).unwrap();

        assert!(manifest_path.exists());
    }

    #[test]
    fn fails_if_manifest_exists() {
        let temp = TempDir::new().unwrap();
        let manifest_path = temp.path().join(MANIFEST_FILE);
        fs::write(&manifest_path, "").unwrap();

        let result = init_at(&manifest_path);

        assert!(matches!(result, Err(Error::AlreadyExists(_))));
    }
}
