mod cli;
mod error;
mod files;
mod handlers;
mod manifest;
mod path_ext;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    cli::parse()?;

    Ok(())
}
