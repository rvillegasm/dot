use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml::{de, ser};

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

pub type Manifest = HashMap<PathBuf, PathBuf>;

pub fn load(buffer: &str) -> Result<Manifest, de::Error> {
    toml::from_str(buffer)
}

pub fn save(manifest: &Manifest) -> Result<String, ser::Error> {
    toml::to_string(manifest)
}

pub fn insert(manifest: &Manifest, file: PathBuf, link: PathBuf) -> Manifest {
    let mut updated_config = manifest.clone();
    updated_config.insert(file, link);
    updated_config
}

pub fn remove(manifest: &Manifest, file: &PathBuf) -> Manifest {
    let mut updated_config = manifest.clone();
    updated_config.remove(file);
    updated_config
}

pub fn get(manifest: &Manifest, file: &Path) -> Option<PathBuf> {
    manifest.get(file).map(PathBuf::to_owned)
}

pub fn has(manifest: &Manifest, file: &Path) -> bool {
    manifest.get(file).is_some()
}
