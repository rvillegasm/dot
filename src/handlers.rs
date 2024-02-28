use std::{io, path::PathBuf};

use crate::{config, error, files};

pub fn add(file_path: &str) -> io::Result<()> {
    let original_file_path = PathBuf::from(file_path);
    let destination_file_path = PathBuf::from(
        original_file_path
            .file_name()
            .ok_or_else(error::not_found)?,
    );

    let config_path = PathBuf::from(config::CONFIG_FILE_NAME);
    let config = config::load(&files::read(&config_path)?).map_err(error::from_other)?;

    if config::has(&config, &destination_file_path) {
        println!("File already being tracked");
        return Ok(());
    }

    files::rename(&original_file_path, &destination_file_path)?;
    let link = files::symlink(&destination_file_path, &original_file_path)?;

    let config = config::insert(&config, link.from, link.to);
    let config_buffer = config::save(&config).map_err(error::from_other)?;

    files::write(&config_path, &config_buffer)?;

    Ok(())
}

pub fn sync() -> io::Result<()> {
    let config_path = PathBuf::from(config::CONFIG_FILE_NAME);
    let config = config::load(&files::read(&config_path)?).map_err(error::from_other)?;

    for (current_path, path_to_symlink) in config.iter() {
        if !files::exists(path_to_symlink) {
            files::create_parent_path(path_to_symlink)?;

            let link = files::symlink(current_path, path_to_symlink)?;

            println!("Linked {} to {}", link.from.display(), link.to.display())
        } else if !files::is_symlink(path_to_symlink)? {
            println!(
                "Found file {}. Please remove it to sync tracked version",
                path_to_symlink.display(),
            )
        }
    }

    Ok(())
}
