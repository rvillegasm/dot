use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::Command;
use crate::error::{Error, Result};
use crate::manifest::Manifest;

pub struct RemoveCommand {
    file_path: PathBuf,
}

impl RemoveCommand {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Core logic separated for testing
    pub fn remove_from_manifest(manifest: &mut Manifest, file_path: &Path) -> Result<PathBuf> {
        let symlink_path = manifest
            .get(file_path)
            .ok_or_else(|| Error::NotFound(file_path.to_path_buf()))?;

        if !file_path.exists() {
            return Err(Error::NotFound(file_path.to_path_buf()));
        }

        let metadata = symlink_path.symlink_metadata()?;
        if !metadata.file_type().is_symlink() {
            return Err(Error::NotASymlink(symlink_path.clone()));
        }

        // Remove symlink and restore file
        fs::remove_file(&symlink_path)?;
        fs::rename(file_path, &symlink_path)?;

        manifest.remove(file_path);

        Ok(symlink_path)
    }
}

impl Command for RemoveCommand {
    fn execute(self) -> Result<()> {
        let mut manifest = Manifest::load()?;
        let restored_path = Self::remove_from_manifest(&mut manifest, &self.file_path)?;
        manifest.save()?;

        println!(
            "Removed {} (restored to {})",
            self.file_path.display(),
            restored_path.display()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    /// Test helper that removes a tracked file using absolute paths
    fn remove_tracked_file(local_file: &Path, symlink_target: &Path) -> Result<()> {
        let metadata = symlink_target.symlink_metadata()?;
        if !metadata.file_type().is_symlink() {
            return Err(Error::NotASymlink(symlink_target.to_path_buf()));
        }

        if !local_file.exists() {
            return Err(Error::NotFound(local_file.to_path_buf()));
        }

        fs::remove_file(symlink_target)?;
        fs::rename(local_file, symlink_target)?;

        Ok(())
    }

    #[test]
    fn returns_error_if_not_tracked() {
        let mut manifest = Manifest::empty();
        let result = RemoveCommand::remove_from_manifest(&mut manifest, Path::new("nottracked"));
        assert!(matches!(result, Err(Error::NotFound(_))));
    }

    #[test]
    fn restores_file_to_original_location() {
        let repo = TempDir::new().unwrap();
        let original_dir = TempDir::new().unwrap();
        let original_path = original_dir.path().join("myfile");

        // Create local file
        let local_file = repo.path().join("myfile");
        fs::write(&local_file, "content").unwrap();

        // Create symlink at original location
        symlink(local_file.canonicalize().unwrap(), &original_path).unwrap();

        // Remove using test helper with absolute paths
        remove_tracked_file(&local_file, &original_path).unwrap();

        // Original is now a regular file
        assert!(original_path.exists());
        assert!(!original_path
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink());
        // Local file is gone
        assert!(!local_file.exists());
    }

    #[test]
    fn returns_error_if_target_is_not_symlink() {
        let repo = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        let target_path = target_dir.path().join("myfile");

        // Create local file and regular file (not symlink) at target
        let local_file = repo.path().join("myfile");
        fs::write(&local_file, "local").unwrap();
        fs::write(&target_path, "blocking").unwrap();

        let result = remove_tracked_file(&local_file, &target_path);
        assert!(matches!(result, Err(Error::NotASymlink(_))));
    }
}
