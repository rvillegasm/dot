mod commands;
mod config;
mod error;
mod files;

fn main() -> std::io::Result<()> {
    commands::add("../test.txt")?;

    Ok(()) // TODO: Manage error correctly later
}
