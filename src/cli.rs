use std::io;

use clap::{Parser, Subcommand};

use crate::handlers;

#[derive(Parser)]
#[command(version)]
#[command(about = "A simple configuration files (i.e. dotfiles) manager ", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Configures the current directory to be able to track new files
    Init,
    /// Tracks a new file/directory by moving it to the current directory and creating a symlink to its
    /// original location
    Add {
        /// Path to the file/directory to track
        path: String,
    },
    /// Removes a file from the manifest, deletes the symlink and returns it to the original
    /// location
    Remove {
        /// Path to the file/directory to remove
        path: String,
    },
    /// Goes through the manifest and creates symlinks for each file
    Sync,
}

pub fn parse() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init => handlers::init(),
        Command::Add { path: file_path } => handlers::add(file_path),
        Command::Remove { path: file_path } => handlers::remove(file_path),
        Command::Sync => handlers::sync(),
    }
}
