mod cli;
mod commands;
mod error;
mod fs;
mod manifest;
mod output;
mod path_ext;
mod service;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    cli::parse()?;

    Ok(())
}
