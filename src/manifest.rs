use std::{
    collections::{BTreeMap, btree_map::Iter},
    path::{Path, PathBuf},
};

use crate::{
    error::DotError,
    fs::{FileSystem, symlink::SymLink},
    path_ext::HomeTildePathTransformer,
};

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

/// A trait that defines operations for managing a dot file manifest
pub trait ManifestOperations {
    /// Check if a file is tracked in the manifest
    fn has_file<P: AsRef<Path>>(&self, file: P) -> bool;

    /// Get the symlink path for a tracked file
    fn get_symlink_path<P: AsRef<Path>>(&self, file: P) -> Option<PathBuf>;

    /// Iterate over all tracked files and their symlink paths
    fn iter_tracked_files(&self) -> Iter<'_, PathBuf, PathBuf>;

    /// Insert a new symlink into the manifest
    fn insert_symlink(&mut self, symlink: &SymLink) -> Result<(), DotError>;

    /// Remove a file from the manifest
    fn remove_file<P: AsRef<Path>>(&mut self, file: P) -> bool;

    /// Serialize the manifest to a string
    fn serialize(&self) -> Result<String, DotError>;

    /// Save the manifest to a file
    fn save(&self, fs: &impl FileSystem) -> Result<(), DotError>;
}

/// Implementation of a manifest that stores dot file tracking information
pub struct Manifest {
    entries: BTreeMap<PathBuf, PathBuf>,
}

impl Manifest {
    /// Create a new manifest by reading from the filesystem
    pub fn from_disk(fs: &impl FileSystem) -> Result<Self, DotError> {
        let manifest_path = Path::new(MANIFEST_FILE_NAME);
        if fs.exists(manifest_path) {
            let content = fs.read(manifest_path)?;
            Self::new(&content)
        } else {
            Ok(Self::empty())
        }
    }

    /// Create a new manifest from a TOML string
    pub fn new(buffer: &str) -> Result<Manifest, DotError> {
        let entries: BTreeMap<PathBuf, PathBuf> = toml::from_str(buffer)?;
        Ok(Manifest { entries })
    }

    /// Create an empty manifest
    pub fn empty() -> Self {
        Manifest {
            entries: BTreeMap::new(),
        }
    }
}

impl ManifestOperations for Manifest {
    fn has_file<P: AsRef<Path>>(&self, file: P) -> bool {
        self.entries.contains_key(file.as_ref())
    }

    fn get_symlink_path<P: AsRef<Path>>(&self, file: P) -> Option<PathBuf> {
        let stored_path = self.entries.get(file.as_ref());

        match stored_path {
            Some(path_str) if path_str.starts_with("~") => {
                dirs::home_dir().map(|home_dir| home_dir.join(path_str.strip_prefix("~").unwrap()))
            }
            _ => stored_path.map(|p| p.to_owned()),
        }
    }

    fn iter_tracked_files(&self) -> Iter<'_, PathBuf, PathBuf> {
        self.entries.iter()
    }

    fn insert_symlink(&mut self, symlink: &SymLink) -> Result<(), DotError> {
        let modified_link = symlink.to.transform_to_tilde_path()?;
        self.entries.insert(symlink.from.to_owned(), modified_link);
        Ok(())
    }

    fn remove_file<P: AsRef<Path>>(&mut self, file: P) -> bool {
        self.entries.remove(file.as_ref()).is_some()
    }

    fn serialize(&self) -> Result<String, DotError> {
        toml::to_string(&self.entries).map_err(DotError::TomlSer)
    }

    fn save(&self, fs: &impl FileSystem) -> Result<(), DotError> {
        let manifest_path = Path::new(MANIFEST_FILE_NAME);
        let manifest_content = self.serialize()?;
        fs.write(manifest_path, &manifest_content)
    }
}

// Tests will be added once we add tempfile as a development dependency
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_manifest_empty() {
        let manifest = Manifest::empty();
        assert_eq!(manifest.entries.len(), 0);
    }

    #[test]
    fn insert_and_get_symlink_roundtrip() {
        use crate::fs::symlink::SymLink;
        use crate::path_ext::HomeTildePathTransformer;

        let mut manifest = Manifest::empty();
        let home = dirs::home_dir().expect("home");
        let original = Path::new("config.txt");
        let target = home.join(".config/config.txt");

        let link = SymLink::new(original, &target);
        manifest.insert_symlink(&link).expect("insert");

        // The stored path in the manifest should use a tilde prefix
        let expected_tilde = target.transform_to_tilde_path().unwrap();
        assert_eq!(manifest.entries.get(original).unwrap(), &expected_tilde);

        // When requesting via the API it should expand back to the full absolute path
        assert_eq!(manifest.get_symlink_path(original).unwrap(), target);
    }

    #[test]
    fn remove_file_returns_true_when_present() {
        let mut manifest = Manifest::empty();
        manifest
            .entries
            .insert(PathBuf::from("a"), PathBuf::from("b"));
        assert!(manifest.remove_file("a"));
        assert!(!manifest.has_file("a"));
    }

    #[test]
    fn serialize_and_deserialize_consistency() {
        let mut original = Manifest::empty();
        original
            .entries
            .insert(PathBuf::from("a"), PathBuf::from("b"));
        let toml = original.serialize().expect("serialize");
        let parsed = Manifest::new(&toml).expect("parse");
        assert!(parsed.has_file("a"));
    }
}
