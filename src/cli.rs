use clap::{Parser, Subcommand};
use std::path::Path;

use crate::{
    commands::{AddCommand, ConsoleOutput, InitCommand, RemoveCommand, SyncCommand},
    error::DotError,
    fs::{
        FileSystem,
        operations::StdFileSystem,
        symlink::UnixSymLinkOperations,
    },
    manifest::{Manifest, MANIFEST_FILE_NAME},
    service::DotCommand,
};

#[derive(Parser)]
#[command(version)]
#[command(about = "A simple configuration files (i.e. dotfiles) manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Configures the current directory to be able to track new files by adding a manifest file
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

/// Create a new manifest for the application
fn create_manifest() -> Result<Manifest, DotError> {
    let fs = StdFileSystem::default();
    let manifest_path = Path::new(MANIFEST_FILE_NAME);
    
    if fs.exists(manifest_path) {
        let content = fs.read(manifest_path)?;
        Manifest::new(&content)
    } else {
        Ok(Manifest::empty())
    }
}

pub fn parse() -> Result<(), DotError> {
    let cli = Cli::parse();
    let fs = StdFileSystem::default();
    let symlink_ops = UnixSymLinkOperations::new(fs.clone());
    
    match &cli.command {
        Command::Init => {
            let manifest = Manifest::empty();
            let service = crate::service::DotService::new(fs, symlink_ops, manifest);
            let output = ConsoleOutput;
            let command = InitCommand::new(service, output);
            command.execute()
        }
        Command::Add { path } => {
            let manifest = create_manifest()?;
            let service = crate::service::DotService::new(fs, symlink_ops, manifest);
            let output = ConsoleOutput;
            let command = AddCommand::new(service, output, path.clone());
            command.execute()
        }
        Command::Remove { path } => {
            let manifest = create_manifest()?;
            let service = crate::service::DotService::new(fs, symlink_ops, manifest);
            let output = ConsoleOutput;
            let command = RemoveCommand::new(service, output, path.clone());
            command.execute()
        }
        Command::Sync => {
            let manifest = create_manifest()?;
            let service = crate::service::DotService::new(fs, symlink_ops, manifest);
            let output = ConsoleOutput;
            let command = SyncCommand::new(service, output);
            command.execute()
        }
    }
}
