use std::io;

pub type Error = io::Error;

pub fn not_found() -> Error {
    io::Error::new(io::ErrorKind::NotFound, "File name not found")
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
