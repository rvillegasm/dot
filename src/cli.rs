use std::io;

use clap::{Parser, Subcommand};

use crate::handlers;

#[derive(Parser)]
#[command(version)]
#[command(about = "A simple configuration files (i.e. dotfiles) manager ", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tracks a new file/directory by moving it to the current directory and creating a symlink to its
    /// original location
    Add {
        /// Path to the file/directory to track
        path: String,
    },
}

pub fn parse() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path: file_path } => handlers::add(&file_path),
    }
}
