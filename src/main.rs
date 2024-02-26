mod cli;
mod config;
mod error;
mod files;
mod handlers;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    cli::parse()?;

    Ok(())
}
