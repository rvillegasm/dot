use std::{io, path::PathBuf};

use crate::{error, files, manifest};

pub fn init() -> io::Result<()> {
    let manifest_path = PathBuf::from(manifest::MANIFEST_FILE_NAME);

    if files::exists(&manifest_path) {
        return Err(error::already_exists(&manifest_path));
    }

    files::write(&manifest_path, "")?;

    Ok(())
}

pub fn add(file_path: &str) -> io::Result<()> {
    let original_file_path = PathBuf::from(file_path);
    let destination_file_path = PathBuf::from(
        original_file_path
            .file_name()
            .ok_or_else(|| error::not_found(&original_file_path))?,
    );

    let manifest_path = PathBuf::from(manifest::MANIFEST_FILE_NAME);
    let manifest = manifest::load(&files::read(&manifest_path)?).map_err(error::from_other)?;

    if manifest::has(&manifest, &destination_file_path) {
        return Err(error::already_tracked(&destination_file_path));
    }

    files::rename(&original_file_path, &destination_file_path)?;
    let link = files::symlink(&destination_file_path, &original_file_path)?;

    let manifest = manifest::insert(&manifest, link.from, link.to);
    let manifest_buffer = manifest::save(&manifest).map_err(error::from_other)?;

    files::write(&manifest_path, &manifest_buffer)?;

    println!(
        "{} -> {}",
        &destination_file_path.display(),
        &original_file_path.display()
    );

    Ok(())
}

pub fn remove(file_path: &str) -> io::Result<()> {
    let file_to_remove = PathBuf::from(file_path);

    let manifest_path = PathBuf::from(manifest::MANIFEST_FILE_NAME);
    let manifest = manifest::load(&files::read(&manifest_path)?).map_err(error::from_other)?;

    let destination_file_path = manifest::get(&manifest, &file_to_remove)
        .ok_or_else(|| error::not_found(&file_to_remove))?;

    let to_remove_exists = files::exists(&file_to_remove)
        .then_some(())
        .ok_or_else(|| error::not_found(&file_to_remove));

    let destination_exists = files::exists(&destination_file_path)
        .then_some(())
        .ok_or_else(|| error::not_found(&destination_file_path));

    if !to_remove_exists
        .and(destination_exists)
        .and(files::is_symlink(&destination_file_path))?
    {
        return Err(error::symlink_not_found(&destination_file_path));
    }

    files::remove(&destination_file_path)?;
    files::rename(&file_to_remove, &destination_file_path)?;

    let updated_manifest = manifest::remove(&manifest, &file_to_remove);
    let updated_manifest_buffer = manifest::save(&updated_manifest).map_err(error::from_other)?;

    files::write(&manifest_path, &updated_manifest_buffer)?;

    Ok(())
}

pub fn sync() -> io::Result<()> {
    let manifest_path = PathBuf::from(manifest::MANIFEST_FILE_NAME);
    let manifest = manifest::load(&files::read(&manifest_path)?).map_err(error::from_other)?;

    for (current_path, path_to_symlink) in manifest.iter() {
        if !files::exists(current_path) {
            return io::Result::Err(error::not_found(current_path));
        }

        if !files::exists(path_to_symlink) {
            files::create_parent_path(path_to_symlink)?;

            let link = files::symlink(current_path, path_to_symlink)?;

            println!("{} -> {}", &link.from.display(), &link.to.display())
        } else if !files::is_symlink(path_to_symlink)? {
            eprintln!(
                "Found file {}. Please remove it to sync tracked version",
                &path_to_symlink.display(),
            )
        }
    }

    Ok(())
}
