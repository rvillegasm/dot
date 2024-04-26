use std::{
    io,
    path::{Component, Path, PathBuf},
};

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
