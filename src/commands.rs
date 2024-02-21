use std::{io, path::PathBuf};

use crate::{config, files};

pub fn add(file_path: &str) -> io::Result<()> {
    let original_file_path = PathBuf::from(file_path);
    let file_name = original_file_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File name not found"))?;

    let destination_file_path = PathBuf::from(file_name);

    let config_path = PathBuf::from(config::CONFIG_FILE_NAME);
    let config = config::load(&files::read(&config_path)?)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if config::has(
        &config,
        file_name.to_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "File name is not valid UTF-8")
        })?,
    ) {
        println!("File already being tracked");
        return Ok(());
    }

    files::rename(&original_file_path, &destination_file_path)?;
    let link = files::symlink(&destination_file_path, &original_file_path)?;

    let config = config::insert(
        &config,
        link.from.to_str().unwrap().to_owned(), // TODO: remove unwrap
        link.to.to_str().unwrap().to_owned(),   // TODO: remove unwrap, also add full aboslute path
    );

    let config_buffer =
        config::save(&config).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    files::write(&config_path, &config_buffer)?;

    Ok(())
}
