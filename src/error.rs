use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum DotError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("{0} not found")]
    NotFound(PathBuf),
    #[error("{0} already exists")]
    AlreadyExists(PathBuf),
    #[error("{0} is already tracked")]
    AlreadyTracked(PathBuf),
    #[error("Invalid path")]
    InvalidPath,
    #[error("Expected {0} to be a symlink")]
    SymlinkNotFound(PathBuf),
    #[error("Other error: {0}")]
    Other(String),
}
