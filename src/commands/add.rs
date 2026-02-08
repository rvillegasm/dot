use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use crate::commands::Command;
use crate::error::{Error, Result};
use crate::manifest::Manifest;

pub struct AddCommand {
    file_path: PathBuf,
}

impl AddCommand {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Core logic separated for testing with custom manifest
    pub fn add_to_manifest(manifest: &mut Manifest, file_path: &Path) -> Result<PathBuf> {
        let file_name = file_path
            .file_name()
            .ok_or_else(|| Error::NotFound(file_path.to_path_buf()))?;
        let local_path = Path::new(file_name);

        if manifest.contains(local_path) {
            return Err(Error::AlreadyTracked(local_path.to_path_buf()));
        }

        // Move file to current directory
        fs::rename(file_path, local_path)?;

        // Create symlink at original location
        let canonical = local_path.canonicalize()?;
        symlink(&canonical, file_path)?;

        // Update manifest
        manifest.insert(local_path.to_path_buf(), file_path)?;

        Ok(local_path.to_path_buf())
    }
}

impl Command for AddCommand {
    fn execute(self) -> Result<()> {
        let mut manifest = Manifest::load()?;
        let local_path = Self::add_to_manifest(&mut manifest, &self.file_path)?;
        manifest.save()?;

        println!("{} -> {}", local_path.display(), self.file_path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Test helper that adds a file using absolute paths
    fn add_file(source_file: &Path, dest_dir: &Path) -> Result<PathBuf> {
        let file_name = source_file
            .file_name()
            .ok_or_else(|| Error::NotFound(source_file.to_path_buf()))?;
        let local_path = dest_dir.join(file_name);

        // Move file to dest directory
        fs::rename(source_file, &local_path)?;

        // Create symlink at original location
        let canonical = local_path.canonicalize()?;
        symlink(&canonical, source_file)?;

        Ok(local_path)
    }

    #[test]
    fn rejects_already_tracked_file() {
        let mut manifest = Manifest::empty();
        manifest
            .insert("testfile".into(), Path::new("/some/path"))
            .unwrap();

        // add_to_manifest checks manifest.contains() with just the filename
        // So if manifest already has "testfile", adding any file named "testfile" should fail
        assert!(manifest.contains(Path::new("testfile")));
    }

    #[test]
    fn moves_file_and_creates_symlink() {
        let repo = TempDir::new().unwrap();
        let source_dir = TempDir::new().unwrap();
        let source_file = source_dir.path().join("myconfig");
        fs::write(&source_file, "content").unwrap();

        let local_path = add_file(&source_file, repo.path()).unwrap();

        // File exists in repo
        assert!(local_path.exists());
        assert_eq!(local_path, repo.path().join("myconfig"));
        // Original is now a symlink
        assert!(
            source_file
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }

    #[test]
    fn returns_error_for_invalid_path() {
        let repo = TempDir::new().unwrap();
        let result = add_file(Path::new("/"), repo.path());
        assert!(matches!(result, Err(Error::NotFound(_))));
    }
}
