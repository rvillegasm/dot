use std::{io, path::Path};

pub type Error = io::Error;

pub fn not_found(path: &Path) -> Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        format!("{} not found", path.display()),
    )
}

pub fn invalid_path() -> Error {
    io::Error::new(io::ErrorKind::InvalidData, "Given path is not supported")
}

pub fn from_other<E>(e: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}
