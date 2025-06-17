use std::{
    collections::{hash_map::Iter, HashMap},
    path::{Path, PathBuf},
};

use crate::{error::DotError, files::SymLink, path_ext::ToLexicalAbsolute};

pub const MANIFEST_FILE_NAME: &str = "dot.toml";

pub struct Manifest(HashMap<PathBuf, PathBuf>);

impl Manifest {
    pub fn new(buffer: &str) -> Result<Manifest, DotError> {
        Ok(Manifest(toml::from_str(buffer)?))
    }

    pub fn save(&self) -> Result<String, DotError> {
        toml::to_string(&self.0).map_err(DotError::TomlSer)
    }

    pub fn insert(&mut self, symlink: &SymLink) -> Result<(), DotError> {
        let home_dir = dirs::home_dir().ok_or(DotError::InvalidPath)?;
        let abs_path_link = symlink.to.to_lexical_absolute()?;

        let modified_link = if abs_path_link.starts_with(&home_dir) {
            Path::new("~").join(abs_path_link.strip_prefix(home_dir).unwrap())
        } else {
            symlink.to.to_owned()
        };

        self.0.insert(symlink.from.to_owned(), modified_link);

        Ok(())
    }

    pub fn remove<P: AsRef<Path>>(&mut self, file: P) -> bool {
        self.0.remove(file.as_ref()).is_some()
    }

    pub fn has<P: AsRef<Path>>(&self, file: P) -> bool {
        self.0.get(file.as_ref()).is_some()
    }

    pub fn get<P: AsRef<Path>>(&self, file: P) -> Option<PathBuf> {
        let stored_relative_path = self.0.get(file.as_ref());

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
