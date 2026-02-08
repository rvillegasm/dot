use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to parse manifest: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Failed to serialize manifest: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("{0} not found")]
    NotFound(PathBuf),
    #[error("{0} already exists")]
    AlreadyExists(PathBuf),
    #[error("{0} is already tracked")]
    AlreadyTracked(PathBuf),
    #[error("Cannot determine home directory")]
    NoHomeDir,
    #[error("Expected {0} to be a symlink")]
    NotASymlink(PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;
