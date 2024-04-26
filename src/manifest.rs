use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use toml::{de, ser};

use crate::{error, path_ext::ToLexicalAbsolute};

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

// TODO: use the newtype pattern and wrap the hashmap
pub type Manifest = HashMap<PathBuf, PathBuf>;

pub fn load(buffer: &str) -> Result<Manifest, de::Error> {
    toml::from_str(buffer)
}

pub fn save(manifest: &Manifest) -> Result<String, ser::Error> {
    toml::to_string(manifest)
}

pub fn insert(manifest: &Manifest, file: PathBuf, link: PathBuf) -> io::Result<Manifest> {
    let home_dir = dirs::home_dir().ok_or_else(error::invalid_path)?;
    let absolute_path_link = link.to_lexical_absolute()?;

    let modified_link = if absolute_path_link.starts_with(&home_dir) {
        Path::new("~").join(absolute_path_link.strip_prefix(home_dir).unwrap())
    } else {
        link
    };

    let mut updated_config = manifest.clone();
    updated_config.insert(file, modified_link);

    Ok(updated_config)
}

pub fn remove(manifest: &Manifest, file: &PathBuf) -> Manifest {
    let mut updated_config = manifest.clone();
    updated_config.remove(file);
    updated_config
}

pub fn get(manifest: &Manifest, file: &Path) -> Option<PathBuf> {
    let stored_relative_path = manifest.get(file);

    match stored_relative_path {
        Some(path) if path.starts_with("~") => {
            dirs::home_dir().map(|home_dir| home_dir.join(path.strip_prefix("~").unwrap()))
        }
        _ => stored_relative_path.map(PathBuf::to_owned),
    }
}

pub fn has(manifest: &Manifest, file: &Path) -> bool {
    manifest.get(file).is_some()
}
