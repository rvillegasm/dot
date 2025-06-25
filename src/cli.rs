use clap::{Parser, Subcommand};

use crate::{
    commands::{AddCommand, DotCommand, InitCommand, RemoveCommand, SyncCommand},
    error::DotError,
    fs::{operations::StdFileSystem, symlink::UnixSymLinkOperations},
    manifest::Manifest,
    output::ConsoleOutput,
    service::DotService,
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

pub fn parse() -> Result<(), DotError> {
    let cli = Cli::parse();
    let fs = StdFileSystem;
    let symlink_ops = UnixSymLinkOperations::new(fs.clone());

    let command: Box<dyn DotCommand> = if let Command::Init = cli.command {
        let manifest = Manifest::empty();
        let service = DotService::new(fs, symlink_ops, manifest);
        Box::new(InitCommand::new(service, ConsoleOutput))
    } else {
        let manifest = Manifest::from_disk(&fs)?;
        let service = DotService::new(fs, symlink_ops, manifest);
        match cli.command {
            Command::Add { path } => Box::new(AddCommand::new(service, ConsoleOutput, path)),
            Command::Remove { path } => Box::new(RemoveCommand::new(service, ConsoleOutput, path)),
            Command::Sync => Box::new(SyncCommand::new(service, ConsoleOutput)),
            Command::Init => unreachable!(),
        }
    };
    command.execute()
}
