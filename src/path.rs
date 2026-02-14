use std::path::{Component, Path, PathBuf};

use crate::error::{Error, Result};

#[allow(dead_code)]
pub fn expand_tilde(path: &Path) -> Result<PathBuf> {
    expand_tilde_with_home(path, dirs::home_dir())
}

#[allow(dead_code)]
pub fn collapse_tilde(path: &Path) -> Result<PathBuf> {
    collapse_tilde_with_home(path, dirs::home_dir())
}

pub fn expand_tilde_with_home(path: &Path, home: Option<PathBuf>) -> Result<PathBuf> {
    if path.starts_with("~") {
        let home = home.ok_or(Error::NoHomeDir)?;
        let suffix = path.strip_prefix("~").expect("checked above");
        Ok(home.join(suffix))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn collapse_tilde_with_home(path: &Path, home: Option<PathBuf>) -> Result<PathBuf> {
    let home = home.ok_or(Error::NoHomeDir)?;
    let abs = to_lexical_absolute(path)?;

    if abs.starts_with(&home) {
        let suffix = abs.strip_prefix(&home).expect("checked above");
        Ok(Path::new("~").join(suffix))
    } else {
        Ok(abs)
    }
}

pub fn to_lexical_absolute(path: &Path) -> Result<PathBuf> {
    let mut absolute = if path.is_absolute() {
        PathBuf::new()
    } else {
        std::env::current_dir()?
    };

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                absolute.pop();
            }
            c => absolute.push(c.as_os_str()),
        }
    }
    Ok(absolute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_with_valid_home() {
        let home = PathBuf::from("/home/user");
        let result = expand_tilde_with_home(Path::new("~/config"), Some(home)).unwrap();
        assert_eq!(result, PathBuf::from("/home/user/config"));
    }

    #[test]
    fn expand_tilde_no_prefix() {
        let result = expand_tilde_with_home(Path::new("/absolute"), Some("/home".into())).unwrap();
        assert_eq!(result, PathBuf::from("/absolute"));
    }

    #[test]
    fn expand_tilde_no_home_errors() {
        let result = expand_tilde_with_home(Path::new("~/config"), None);
        assert!(matches!(result, Err(Error::NoHomeDir)));
    }

    #[test]
    fn collapse_tilde_inside_home() {
        let home = PathBuf::from("/home/user");
        let result = collapse_tilde_with_home(Path::new("/home/user/config"), Some(home)).unwrap();
        assert_eq!(result, PathBuf::from("~/config"));
    }

    #[test]
    fn collapse_tilde_outside_home() {
        let home = PathBuf::from("/home/user");
        let result = collapse_tilde_with_home(Path::new("/etc/config"), Some(home)).unwrap();
        assert_eq!(result, PathBuf::from("/etc/config"));
    }

    #[test]
    fn lexical_absolute_resolves_parent() {
        let result = to_lexical_absolute(Path::new("/foo/bar/../baz")).unwrap();
        assert_eq!(result, PathBuf::from("/foo/baz"));
    }

    #[test]
    fn lexical_absolute_resolves_current() {
        let result = to_lexical_absolute(Path::new("/foo/./bar")).unwrap();
        assert_eq!(result, PathBuf::from("/foo/bar"));
    }
}
