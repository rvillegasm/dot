use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::commands::Command;
use crate::error::{Error, Result};
use crate::manifest::Manifest;

pub struct SyncCommand;

impl SyncCommand {
    pub fn new() -> Self {
        Self
    }

    /// Result of sync operation for testability
    pub fn sync_manifest(manifest: &Manifest) -> Result<SyncResult> {
        let mut result = SyncResult::default();

        for (local_path, symlink_result) in manifest.iter() {
            let symlink_path = symlink_result?;

            if !local_path.exists() {
                return Err(Error::NotFound(local_path.to_path_buf()));
            }

            if !symlink_path.exists() {
                // Create parent directories if needed
                if let Some(parent) = symlink_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let canonical = local_path.canonicalize()?;
                symlink(&canonical, &symlink_path)?;

                result.created.push(CreatedSymlink {
                    local: local_path.to_path_buf(),
                    symlink: symlink_path,
                });
            } else {
                let metadata = symlink_path.symlink_metadata()?;
                if !metadata.file_type().is_symlink() {
                    result.conflicts.push(symlink_path);
                }
            }
        }

        Ok(result)
    }
}

impl Default for SyncCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SyncResult {
    pub created: Vec<CreatedSymlink>,
    pub conflicts: Vec<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub struct CreatedSymlink {
    pub local: PathBuf,
    pub symlink: PathBuf,
}

impl Command for SyncCommand {
    fn execute(self) -> Result<()> {
        let manifest = Manifest::load()?;
        let result = Self::sync_manifest(&manifest)?;

        for created in &result.created {
            log::info!(
                "Created symlink: {} -> {}",
                created.symlink.display(),
                created.local.display()
            );
        }

        for conflict in &result.conflicts {
            log::warn!(
                "{} exists but is not a symlink — skipping",
                conflict.display()
            );
        }

        if result.created.is_empty() && result.conflicts.is_empty() {
            log::info!("Up to date");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink as create_symlink;
    use std::path::Path;
    use tempfile::TempDir;

    fn sync_entries(entries: &[(&Path, &Path)]) -> Result<SyncResult> {
        let mut result = SyncResult::default();

        for (local_path, symlink_path) in entries {
            if !local_path.exists() {
                return Err(Error::NotFound(local_path.to_path_buf()));
            }

            if !symlink_path.exists() {
                if let Some(parent) = symlink_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let canonical = local_path.canonicalize()?;
                symlink(&canonical, symlink_path)?;

                result.created.push(CreatedSymlink {
                    local: local_path.to_path_buf(),
                    symlink: symlink_path.to_path_buf(),
                });
            } else {
                let metadata = symlink_path.symlink_metadata()?;
                if !metadata.file_type().is_symlink() {
                    result.conflicts.push(symlink_path.to_path_buf());
                }
            }
        }

        Ok(result)
    }

    #[test]
    fn creates_missing_symlinks() {
        let repo = TempDir::new().unwrap();
        let local_file = repo.path().join("myfile");
        fs::write(&local_file, "content").unwrap();

        let target_dir = TempDir::new().unwrap();
        let symlink_path = target_dir.path().join("myfile");

        let result = sync_entries(&[(&local_file, &symlink_path)]).unwrap();

        assert_eq!(result.created.len(), 1);
        assert!(
            symlink_path
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }

    #[test]
    fn reports_conflicts() {
        let repo = TempDir::new().unwrap();
        let local_file = repo.path().join("myfile");
        fs::write(&local_file, "content").unwrap();

        let target_dir = TempDir::new().unwrap();
        let conflict_path = target_dir.path().join("myfile");
        fs::write(&conflict_path, "blocking").unwrap();

        let result = sync_entries(&[(&local_file, &conflict_path)]).unwrap();

        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.conflicts[0], conflict_path);
    }

    #[test]
    fn returns_error_for_missing_local_file() {
        let result = sync_entries(&[(Path::new("/nonexistent"), Path::new("/tmp/somewhere"))]);
        assert!(matches!(result, Err(Error::NotFound(_))));
    }

    #[test]
    fn up_to_date_returns_empty_result() {
        let repo = TempDir::new().unwrap();
        let local_file = repo.path().join("myfile");
        fs::write(&local_file, "content").unwrap();

        let target_dir = TempDir::new().unwrap();
        let symlink_path = target_dir.path().join("myfile");
        create_symlink(local_file.canonicalize().unwrap(), &symlink_path).unwrap();

        let result = sync_entries(&[(&local_file, &symlink_path)]).unwrap();

        assert!(result.created.is_empty());
        assert!(result.conflicts.is_empty());
    }
}
