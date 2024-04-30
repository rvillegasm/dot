use std::{io, path::Path};

use crate::{error, files, manifest, manifest::Manifest};

pub fn init() -> io::Result<()> {
    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);

    if files::exists(manifest_path) {
        return Err(error::already_exists(manifest_path));
    }

    files::write(manifest_path, "")?;

    Ok(())
}

pub fn add(file_path: &str) -> io::Result<()> {
    let original_file_path = Path::new(file_path);
    let pwd_file_path = Path::new(
        original_file_path
            .file_name()
            .ok_or_else(|| error::not_found(original_file_path))?,
    );

    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);

    let manifest = Manifest::new(&files::read(manifest_path)?).map_err(error::from_other)?;

    if manifest.has(pwd_file_path) {
        return Err(error::already_tracked(pwd_file_path));
    }

    files::rename(original_file_path, pwd_file_path)?;
    let symlink = files::symlink(pwd_file_path, original_file_path)?;

    let manifest_buffer = manifest
        .insert(&symlink)?
        .save()
        .map_err(error::from_other)?;

    files::write(manifest_path, &manifest_buffer)?;

    println!(
        "{} -> {}",
        pwd_file_path.display(),
        original_file_path.display()
    );

    Ok(())
}

pub fn remove(file_path: &str) -> io::Result<()> {
    let file_to_remove = Path::new(file_path);

    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    let manifest = Manifest::new(&files::read(manifest_path)?).map_err(error::from_other)?;

    let destination_file_path = manifest
        .get(file_to_remove)
        .ok_or_else(|| error::not_found(file_to_remove))?;

    let to_remove_exists = files::exists(file_to_remove)
        .then_some(())
        .ok_or_else(|| error::not_found(file_to_remove));

    let destination_exists = files::exists(&destination_file_path)
        .then_some(())
        .ok_or_else(|| error::not_found(&destination_file_path));

    // TODO: Improve the checking here to make it simpler
    if !to_remove_exists
        .and(destination_exists)
        .and(files::is_symlink(&destination_file_path))?
    {
        return Err(error::symlink_not_found(&destination_file_path));
    }

    files::remove(&destination_file_path)?;
    files::rename(file_to_remove, &destination_file_path)?;

    let updated_manifest_buffer = manifest
        .remove(file_to_remove)
        .save()
        .map_err(error::from_other)?;

    files::write(manifest_path, &updated_manifest_buffer)?;

    println!("Removed {}", &file_to_remove.display());

    Ok(())
}

pub fn sync() -> io::Result<()> {
    let manifest_path = Path::new(manifest::MANIFEST_FILE_NAME);
    let manifest = Manifest::new(&files::read(manifest_path)?).map_err(error::from_other)?;

    let mut up_to_date = true;

    for (current_path, path_to_symlink) in manifest.iter() {
        if !files::exists(current_path) {
            return io::Result::Err(error::not_found(current_path));
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
