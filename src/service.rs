use crate::{
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::{MANIFEST_FILE_NAME, ManifestOperations},
    path_ext::HomeTildePathTransformer,
};

use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    /// In-memory implementation of the `FileSystem` trait for testing purposes.
    #[derive(Debug, Default)]
    struct InMemoryFileSystem {
        files: RefCell<HashMap<PathBuf, String>>, // maps path -> contents (empty string for non-regular files)
        cwd: PathBuf,
    }

    impl InMemoryFileSystem {
        fn new() -> Self {
            Self {
                files: RefCell::new(HashMap::new()),
                cwd: PathBuf::from("/tmp"),
            }
        }
    }

    impl FileSystem for InMemoryFileSystem {
        fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
            self.files
                .borrow()
                .contains_key(&path.as_ref().to_path_buf())
        }

        fn read<P: AsRef<Path>>(&self, path: P) -> Result<String, DotError> {
            self.files
                .borrow()
                .get(&path.as_ref().to_path_buf())
                .cloned()
                .ok_or_else(|| DotError::NotFound(path.as_ref().to_path_buf()))
        }

        fn write<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), DotError> {
            self.files
                .borrow_mut()
                .insert(path.as_ref().to_path_buf(), content.to_owned());
            Ok(())
        }

        fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), DotError> {
            if self
                .files
                .borrow_mut()
                .remove(&path.as_ref().to_path_buf())
                .is_some()
            {
                Ok(())
            } else {
                Err(DotError::NotFound(path.as_ref().to_path_buf()))
            }
        }

        fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), DotError> {
            let mut files = self.files.borrow_mut();
            let content = files
                .remove(&from.as_ref().to_path_buf())
                .ok_or_else(|| DotError::NotFound(from.as_ref().to_path_buf()))?;
            files.insert(to.as_ref().to_path_buf(), content);
            Ok(())
        }

        fn create_parent_path<P: AsRef<Path>>(&self, _path: P) -> Result<(), DotError> {
            // No-op for in-memory representation
            Ok(())
        }

        fn current_dir(&self) -> Result<PathBuf, DotError> {
            Ok(self.cwd.clone())
        }
    }

    #[derive(Debug, Clone, Default)]
    struct InMemorySymLinkOperations {
        links: Rc<RefCell<HashSet<PathBuf>>>, // shared set of symlink destinations
    }

    impl InMemorySymLinkOperations {
        fn new() -> Self {
            Self {
                links: Rc::new(RefCell::new(HashSet::new())),
            }
        }

        /// Helper used in assertions.
        fn is_tracked<P: AsRef<Path>>(&self, path: P) -> bool {
            self.links.borrow().contains(path.as_ref())
        }

        fn clear_all(&self) {
            self.links.borrow_mut().clear();
        }
    }

    impl SymLinkOperations for InMemorySymLinkOperations {
        fn create_symlink<F: AsRef<Path>, T: AsRef<Path>>(
            &self,
            _from: F,
            to: T,
        ) -> Result<crate::fs::symlink::SymLink, DotError> {
            self.links.borrow_mut().insert(to.as_ref().to_path_buf());
            Ok(crate::fs::symlink::SymLink::new(
                _from.as_ref().to_path_buf(),
                to.as_ref().to_path_buf(),
            ))
        }

        fn is_symlink<P: AsRef<Path>>(&self, path: P) -> Result<bool, DotError> {
            Ok(self.links.borrow().contains(path.as_ref()))
        }
    }

    #[test]
    fn init_creates_manifest_file_successfully() -> Result<(), DotError> {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let service = DotService::new(&fs, symlink_ops.clone(), manifest);
        service.init()?;
        assert!(fs.exists(MANIFEST_FILE_NAME));
        Ok(())
    }

    #[test]
    fn init_errors_when_manifest_already_exists() {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let service = DotService::new(&fs, symlink_ops.clone(), manifest);
        fs.write(MANIFEST_FILE_NAME, "existing").unwrap();
        let result = service.init();
        assert!(matches!(result, Err(DotError::AlreadyExists(_))));
    }

    #[test]
    fn add_tracks_new_file_and_updates_manifest() -> Result<(), DotError> {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let mut service = DotService::new(&fs, symlink_ops.clone(), manifest);
        let original_path = "/tmp/example.txt";
        fs.write(original_path, "hello")?;

        service.add(original_path)?;

        // After add, file should exist in current directory (renamed)
        assert!(fs.exists("example.txt"));
        // A symlink should have been created at the original location
        assert!(symlink_ops.is_tracked(original_path));
        // Service should now report that everything is up-to-date
        assert!(service.is_up_to_date()?);
        Ok(())
    }

    #[test]
    fn add_errors_when_file_is_already_tracked() {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let mut service = DotService::new(&fs, symlink_ops.clone(), manifest);
        let tracked_file = "foo.txt";
        // Pretend the manifest already has the entry by calling add once.
        fs.write(tracked_file, "data").unwrap();
        service.add(tracked_file).unwrap();

        // Now try to add again – should error out.
        let result = service.add(tracked_file);
        assert!(matches!(result, Err(DotError::AlreadyTracked(_))));
    }

    #[test]
    fn remove_restores_original_file_and_updates_manifest() -> Result<(), DotError> {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let mut service = DotService::new(&fs, symlink_ops.clone(), manifest);
        let original_path = "/tmp/remove_me.txt";
        fs.write(original_path, "bye")?;
        service.add(original_path)?; // Now tracked
        // Simulate underlying symlink file existing in the file system
        fs.write(original_path, "link_placeholder")?;

        // Preconditions
        assert!(symlink_ops.is_tracked(original_path));

        // Remove the tracked file
        service.remove("remove_me.txt")?;

        // Original file restored at original path
        assert!(fs.exists(original_path));
        // The file should no longer be in current directory
        assert!(!fs.exists("remove_me.txt"));
        Ok(())
    }

    #[test]
    fn sync_creates_missing_symlinks() -> Result<(), DotError> {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let mut service = DotService::new(&fs, symlink_ops.clone(), manifest);
        let original_path = "/tmp/sync_me.txt";
        fs.write(original_path, "content")?;
        service.add(original_path)?; // symlink gets created and tracked

        // Manually remove the symlink to simulate missing link
        symlink_ops.clear_all();
        assert!(!symlink_ops.is_tracked(original_path));

        service.sync()?;
        // After sync the link should be recreated
        assert!(symlink_ops.is_tracked(original_path));
        Ok(())
    }

    #[test]
    fn is_up_to_date_detects_conflicting_files() -> Result<(), DotError> {
        let fs = InMemoryFileSystem::new();
        let symlink_ops = InMemorySymLinkOperations::new();
        let manifest = crate::manifest::Manifest::empty();
        let mut service = DotService::new(&fs, symlink_ops.clone(), manifest);
        let original_path = "/tmp/conflict.txt";
        fs.write(original_path, "data")?;
        service.add(original_path)?;

        // Replace symlink with a regular file to simulate conflict
        symlink_ops.clear_all();
        fs.write(original_path, "regular file")?;

        assert!(!service.is_up_to_date()?);
        Ok(())
    }
}
