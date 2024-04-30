use std::{
    collections::{hash_map::Iter, HashMap},
    io,
    path::{Path, PathBuf},
};

use toml::{de, ser};

use crate::{error, files::SymLink, path_ext::ToLexicalAbsolute};

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

pub struct Manifest(HashMap<PathBuf, PathBuf>);

impl Manifest {
    pub fn new(buffer: &str) -> Result<Manifest, de::Error> {
        Ok(Manifest(toml::from_str(buffer)?))
    }

    pub fn save(&self) -> Result<String, ser::Error> {
        toml::to_string(&self.0)
    }

    pub fn insert(mut self, symlink: &SymLink) -> io::Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(error::invalid_path)?;
        let absolute_path_link = symlink.to.to_lexical_absolute()?;

        let modified_link = if absolute_path_link.starts_with(&home_dir) {
            Path::new("~").join(absolute_path_link.strip_prefix(home_dir).unwrap())
        } else {
            symlink.to.to_owned()
        };

        self.0.insert(symlink.from.to_owned(), modified_link);

        Ok(self)
    }

    pub fn remove(mut self, file: &Path) -> Self {
        self.0.remove(file);

        self
    }

    pub fn has(&self, file: &Path) -> bool {
        self.0.get(file).is_some()
    }

    pub fn get(&self, file: &Path) -> Option<PathBuf> {
        let stored_relative_path = self.0.get(file);

        match stored_relative_path {
            Some(path) if path.starts_with("~") => {
                dirs::home_dir().map(|home_dir| home_dir.join(path.strip_prefix("~").unwrap()))
            }
            _ => stored_relative_path.map(PathBuf::to_owned),
        }
    }

    pub fn iter(&self) -> Iter<'_, PathBuf, PathBuf> {
        self.0.iter()
    }
}
