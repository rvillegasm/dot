use std::path::Path;

use crate::{
    error::DotError,
    files,
    manifest::{self, Manifest},
};

pub fn init() -> Result<(), DotError> {
    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    if files::exists(manifest_path) {
        return Err(DotError::AlreadyExists(manifest_path.to_path_buf()));
    }
    files::write(manifest_path, "")?;
    Ok(())
}

pub fn add(file_path: &str) -> Result<(), DotError> {
    let original_file_path = Path::new(file_path);
    let pwd = Path::new(
        original_file_path
            .file_name()
            .ok_or_else(|| DotError::NotFound(original_file_path.to_path_buf()))?,
    );

    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    let mut manifest =
        Manifest::new(&files::read(manifest_path)?).map_err(|e| DotError::Other(e.to_string()))?;

    if manifest.has(pwd) {
        return Err(DotError::AlreadyTracked(pwd.to_path_buf()));
    }

    files::rename(original_file_path, pwd)?;
    let symlink = files::symlink(pwd, original_file_path)?;
    manifest.insert(&symlink)?;
    let manifest_buffer = manifest.save()?;

    files::write(manifest_path, &manifest_buffer)?;
    println!("{} -> {}", pwd.display(), original_file_path.display());
    Ok(())
}

pub fn remove(file_path: &str) -> Result<(), DotError> {
    let file_to_remove = Path::new(file_path);

    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    let mut manifest =
        Manifest::new(&files::read(manifest_path)?).map_err(|e| DotError::Other(e.to_string()))?;

    let destination_file_path = manifest
        .get(file_to_remove)
        .ok_or_else(|| DotError::NotFound(file_to_remove.to_path_buf()))?;

    if !files::exists(file_to_remove) {
        return Err(DotError::NotFound(file_to_remove.to_path_buf()));
    }

    if !files::exists(&destination_file_path) {
        return Err(DotError::NotFound(destination_file_path.to_path_buf()));
    }

    if !files::is_symlink(&destination_file_path)? {
        return Err(DotError::SymlinkNotFound(
            destination_file_path.to_path_buf(),
        ));
    }

    files::remove(&destination_file_path)?;
    files::rename(file_to_remove, &destination_file_path)?;

    manifest.remove(file_to_remove);
    let updated_manifest_buffer = manifest.save()?;

    files::write(manifest_path, &updated_manifest_buffer)?;
    println!("Removed {}", &file_to_remove.display());
    Ok(())
}

pub fn sync() -> Result<(), DotError> {
    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    let manifest =
        Manifest::new(&files::read(manifest_path)?).map_err(|e| DotError::Other(e.to_string()))?;

    let mut up_to_date = true;

    for (current_path, path_to_symlink) in manifest.iter() {
        if !files::exists(current_path) {
            return Err(DotError::NotFound(current_path.to_path_buf()));
        }

        if !files::exists(path_to_symlink) {
            files::create_parent_path(path_to_symlink)?;

            let link = files::symlink(current_path, path_to_symlink)?;

            println!("{} -> {}", &link.from.display(), &link.to.display())
        } else if !files::is_symlink(path_to_symlink)? {
            up_to_date = false;
            eprintln!(
                "Found file {}. Please remove it to sync tracked version",
                &path_to_symlink.display(),
            )
        }
    }

    if up_to_date {
        println!("Up to date");
    }

    Ok(())
}
