use std::path::Path;

use crate::{
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::{MANIFEST_FILE_NAME, ManifestOperations},
    path_ext::HomeTildePathTransformer,
};

/// Service for managing dot files
pub struct DotService<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations> {
    fs: &'a F,
    symlink_ops: S,
    manifest: M,
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations> DotService<'a, F, S, M> {
    pub fn new(fs: &'a F, symlink_ops: S, manifest: M) -> Self {
        Self {
            fs,
            symlink_ops,
            manifest,
        }
    }

    /// Initialize a new dot repository
    pub fn init(&self) -> Result<(), DotError> {
        let manifest_path = Path::new(MANIFEST_FILE_NAME);
        if self.fs.exists(manifest_path) {
            return Err(DotError::AlreadyExists(manifest_path.to_path_buf()));
        }
        self.fs.write(manifest_path, "")?;
        Ok(())
    }

    /// Add a new file to track
    pub fn add<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), DotError> {
        let original_file_path = file_path.as_ref();
        let pwd_file_name = Path::new(
            original_file_path
                .file_name()
                .ok_or_else(|| DotError::NotFound(original_file_path.to_path_buf()))?,
        );

        if self.manifest.has_file(pwd_file_name) {
            return Err(DotError::AlreadyTracked(pwd_file_name.to_path_buf()));
        }

        self.fs.rename(original_file_path, pwd_file_name)?;
        let symlink = self
            .symlink_ops
            .create_symlink(pwd_file_name, original_file_path)?;

        self.manifest.insert_symlink(&symlink)?;
        self.save_manifest()?;

        Ok(())
    }

    /// Remove a tracked file
    pub fn remove<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), DotError> {
        let file_to_remove = file_path.as_ref();
        let destination_file_path = self
            .manifest
            .get_symlink_path(file_to_remove)
            .ok_or_else(|| DotError::NotFound(file_to_remove.to_path_buf()))?;

        if !self.fs.exists(file_to_remove) {
            return Err(DotError::NotFound(file_to_remove.to_path_buf()));
        }

        if !self.fs.exists(&destination_file_path) {
            return Err(DotError::NotFound(destination_file_path.to_path_buf()));
        }

        if !self.symlink_ops.is_symlink(&destination_file_path)? {
            return Err(DotError::SymlinkNotFound(
                destination_file_path.to_path_buf(),
            ));
        }

        self.fs.remove(&destination_file_path)?;
        self.fs.rename(file_to_remove, &destination_file_path)?;
        self.manifest.remove_file(file_to_remove);
        self.save_manifest()?;

        Ok(())
    }

    /// Synchronize all tracked files
    pub fn sync(&self) -> Result<(), DotError> {
        for (current_path, path_to_symlink) in self.manifest.iter_tracked_files() {
            if !self.fs.exists(current_path) {
                return Err(DotError::NotFound(current_path.to_path_buf()));
            }

            let symlink_path = path_to_symlink.transform_from_tilde_path()?;

            if !self.fs.exists(&symlink_path) {
                self.fs.create_parent_path(&symlink_path)?;
                self.symlink_ops
                    .create_symlink(current_path, &symlink_path)?;
            } else if !self.symlink_ops.is_symlink(&symlink_path)? {
                eprintln!(
                    "WARNING: Found file {}. Please remove it to sync tracked version",
                    &symlink_path.display(),
                );
            }
        }

        Ok(())
    }

    /// Save the manifest to disk
    pub fn save_manifest(&self) -> Result<(), DotError> {
        self.manifest.save(self.fs)
    }

    /// Returns whether everything is up to date
    pub fn is_up_to_date(&self) -> Result<bool, DotError> {
        for (_, path_to_symlink) in self.manifest.iter_tracked_files() {
            // Convert relative paths to absolute for consistent handling
            let symlink_path = if !path_to_symlink.is_absolute() {
                match self.fs.current_dir() {
                    Ok(current_dir) => current_dir.join(path_to_symlink),
                    Err(_) => path_to_symlink.clone(),
                }
            } else {
                path_to_symlink.clone()
            };

            if self.fs.exists(&symlink_path) && !self.symlink_ops.is_symlink(&symlink_path)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
