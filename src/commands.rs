use std::{io, path::PathBuf};

use crate::{config, files};

pub fn add(file_path: &str) -> io::Result<()> {
    let original_file_path = PathBuf::from(file_path);
    let file_name = original_file_path.file_name().unwrap(); // TODO: remove unwrap
    let destination_file_path = PathBuf::from(file_name);

    let config_path = PathBuf::from(config::CONFIG_FILE_NAME);
    let config = config::load(&files::read(&config_path)?).unwrap(); // TODO: remove unwrap

    // TODO: make sure the file is not already in the config and being tracked

    files::rename(&original_file_path, &destination_file_path)?;
    let link = files::symlink(&destination_file_path, &original_file_path)?;

    let config = config::insert(
        &config,
        link.from.to_str().unwrap().to_owned(), // TODO: remove unwrap
        link.to.to_str().unwrap().to_owned(),   // TODO: remove unwrap, also add full aboslute path
    );

    let config_buffer = config::save(&config).unwrap(); // TODO: remove unwrap
    files::write(&config_path, &config_buffer)?;

    Ok(())
}
