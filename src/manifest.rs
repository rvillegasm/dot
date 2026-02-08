use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::path::{collapse_tilde_with_home, expand_tilde_with_home};

pub const MANIFEST_FILE: &str = "dot.toml";

#[derive(Debug, Default)]
pub struct Manifest {
    entries: BTreeMap<PathBuf, PathBuf>,
}

impl Manifest {
    pub fn load() -> Result<Self> {
        Self::load_from(Path::new(MANIFEST_FILE))
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Self::parse(&content)
        } else {
            Ok(Self::empty())
        }
    }

    pub fn parse(content: &str) -> Result<Self> {
        let entries: BTreeMap<PathBuf, PathBuf> = toml::from_str(content)?;
        Ok(Self { entries })
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        self.save_to(Path::new(MANIFEST_FILE))
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        let content = self.serialize()?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn serialize(&self) -> Result<String> {
        toml::to_string(&self.entries).map_err(Error::TomlSerialize)
    }

    pub fn contains(&self, file: &Path) -> bool {
        self.entries.contains_key(file)
    }

    pub fn get(&self, file: &Path) -> Option<PathBuf> {
        self.get_with_home(file, dirs::home_dir())
    }

    pub fn get_with_home(&self, file: &Path, home: Option<PathBuf>) -> Option<PathBuf> {
        self.entries
            .get(file)
            .and_then(|p| expand_tilde_with_home(p, home).ok())
    }

    pub fn insert(&mut self, file: PathBuf, target: &Path) -> Result<()> {
        self.insert_with_home(file, target, dirs::home_dir())
    }

    pub fn insert_with_home(
        &mut self,
        file: PathBuf,
        target: &Path,
        home: Option<PathBuf>,
    ) -> Result<()> {
        let tilde_path = collapse_tilde_with_home(target, home)?;
        self.entries.insert(file, tilde_path);
        Ok(())
    }

    pub fn remove(&mut self, file: &Path) -> bool {
        self.entries.remove(file).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Path, Result<PathBuf>)> + '_ {
        self.iter_with_home(dirs::home_dir())
    }

    pub fn iter_with_home(
        &self,
        home: Option<PathBuf>,
    ) -> impl Iterator<Item = (&Path, Result<PathBuf>)> + '_ {
        self.entries
            .iter()
            .map(move |(k, v)| (k.as_path(), expand_tilde_with_home(v, home.clone())))
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_manifest() {
        let m = Manifest::empty();
        assert!(m.is_empty());
    }

    #[test]
    fn parse_valid_toml() {
        let m = Manifest::parse(r#""vimrc" = "~/.vimrc""#).unwrap();
        assert!(m.contains(Path::new("vimrc")));
    }

    #[test]
    fn parse_empty_toml() {
        let m = Manifest::parse("").unwrap();
        assert!(m.is_empty());
    }

    #[test]
    fn parse_invalid_toml_errors() {
        assert!(Manifest::parse("invalid {{{").is_err());
    }

    #[test]
    fn serialize_roundtrip() {
        let mut m = Manifest::empty();
        m.entries.insert("a".into(), "~/b".into());
        let parsed = Manifest::parse(&m.serialize().unwrap()).unwrap();
        assert!(parsed.contains(Path::new("a")));
    }

    #[test]
    fn insert_collapses_tilde() {
        let home = PathBuf::from("/home/user");
        let mut m = Manifest::empty();
        m.insert_with_home("config".into(), Path::new("/home/user/.config"), Some(home))
            .unwrap();
        assert_eq!(
            m.entries.get(Path::new("config")),
            Some(&PathBuf::from("~/.config"))
        );
    }

    #[test]
    fn get_expands_tilde() {
        let home = PathBuf::from("/home/user");
        let mut m = Manifest::empty();
        m.entries.insert("config".into(), "~/.config".into());
        assert_eq!(
            m.get_with_home(Path::new("config"), Some(home)),
            Some(PathBuf::from("/home/user/.config"))
        );
    }

    #[test]
    fn remove_existing() {
        let mut m = Manifest::empty();
        m.entries.insert("a".into(), "b".into());
        assert!(m.remove(Path::new("a")));
        assert!(!m.contains(Path::new("a")));
    }

    #[test]
    fn remove_nonexistent() {
        let mut m = Manifest::empty();
        assert!(!m.remove(Path::new("x")));
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let m = Manifest::empty();
        assert!(m.get(Path::new("x")).is_none());
    }
}
