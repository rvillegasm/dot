use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use toml::{de, ser};

use crate::error;

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

pub type Manifest = HashMap<PathBuf, PathBuf>;

pub fn load(buffer: &str) -> Result<Manifest, de::Error> {
    toml::from_str(buffer)
}

pub fn save(manifest: &Manifest) -> Result<String, ser::Error> {
    toml::to_string(manifest)
}

pub fn insert(manifest: &Manifest, file: PathBuf, link: PathBuf) -> io::Result<Manifest> {
    // TODO: save the path using the dirs crate to save a ~ reference
    let home_dir = dirs::home_dir().ok_or_else(error::invalid_path)?;
    let modified_link = if link.canonicalize()?.starts_with(home_dir) {
    } else {
    };

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
