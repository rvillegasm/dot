use std::{
    io,
    path::{Component, Path, PathBuf},
};

use crate::error::DotError;

pub trait ToLexicalAbsolute {
    fn to_lexical_absolute(&self) -> io::Result<PathBuf>;
}

impl ToLexicalAbsolute for Path {
    fn to_lexical_absolute(&self) -> io::Result<PathBuf> {
        let mut absolute = if self.is_absolute() {
            PathBuf::new()
        } else {
            std::env::current_dir()?
        };
        for component in self.components() {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    absolute.pop();
                }
                component => absolute.push(component.as_os_str()),
            }
        }
        Ok(absolute)
    }
}

pub trait HomeTildePathTransformer {
    fn transform_to_tilde_path(&self) -> Result<PathBuf, DotError>;
    fn transform_from_tilde_path(&self) -> Result<PathBuf, DotError>;
}

impl HomeTildePathTransformer for Path {
    fn transform_to_tilde_path(&self) -> Result<PathBuf, DotError> {
        let home_dir = dirs::home_dir().ok_or(DotError::InvalidPath)?;
        let abs_path = self.to_lexical_absolute()?;

        let home_relative_path = if abs_path.starts_with(&home_dir) {
            Path::new("~").join(abs_path.strip_prefix(&home_dir).unwrap())
        } else {
            self.to_path_buf()
        };

        Ok(home_relative_path)
    }

    fn transform_from_tilde_path(&self) -> Result<PathBuf, DotError> {
        let home_dir = dirs::home_dir().ok_or(DotError::InvalidPath)?;

        let from_home_relative_path = if self.starts_with("~") {
            home_dir.join(self.strip_prefix("~").unwrap())
        } else {
            self.to_path_buf()
        };

        Ok(from_home_relative_path)
    }
}
