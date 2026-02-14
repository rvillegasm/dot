use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::commands::{AddCommand, Command, InitCommand, RemoveCommand, SyncCommand};
use crate::error::Result;

#[derive(Parser)]
#[command(version, about = "A simple dotfiles manager")]
pub struct Cli {
    #[arg(short, long, global = true, help = "Suppress all output")]
    quiet: bool,
    #[arg(short, long, global = true, help = "Enable verbose output")]
    verbose: bool,
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Initialize a new dot repository
    Init,
    /// Track a file by moving it here and creating a symlink
    Add { path: PathBuf },
    /// Stop tracking a file and restore it
    Remove { path: PathBuf },
    /// Create symlinks for all tracked files
    Sync,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.quiet {
        LevelFilter::Off
    } else if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    crate::logger::init(level);

    match cli.command {
        CliCommand::Init => InitCommand::new().execute(),
        CliCommand::Add { path } => AddCommand::new(path).execute(),
        CliCommand::Remove { path } => RemoveCommand::new(path).execute(),
        CliCommand::Sync => SyncCommand::new().execute(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_init() {
        let cli = Cli::try_parse_from(["dot", "init"]).unwrap();
        assert!(matches!(cli.command, CliCommand::Init));
    }

    #[test]
    fn parse_add() {
        let cli = Cli::try_parse_from(["dot", "add", "/path/file"]).unwrap();
        assert!(
            matches!(cli.command, CliCommand::Add { path } if path == PathBuf::from("/path/file"))
        );
    }

    #[test]
    fn parse_remove() {
        let cli = Cli::try_parse_from(["dot", "remove", "myfile"]).unwrap();
        assert!(
            matches!(cli.command, CliCommand::Remove { path } if path == PathBuf::from("myfile"))
        );
    }

    #[test]
    fn parse_sync() {
        let cli = Cli::try_parse_from(["dot", "sync"]).unwrap();
        assert!(matches!(cli.command, CliCommand::Sync));
    }

    #[test]
    fn add_requires_path() {
        assert!(Cli::try_parse_from(["dot", "add"]).is_err());
    }

    #[test]
    fn parse_quiet_flag() {
        let cli = Cli::try_parse_from(["dot", "--quiet", "sync"]).unwrap();
        assert!(cli.quiet);
        assert!(!cli.verbose);
    }

    #[test]
    fn parse_verbose_flag() {
        let cli = Cli::try_parse_from(["dot", "--verbose", "sync"]).unwrap();
        assert!(cli.verbose);
        assert!(!cli.quiet);
    }

    #[test]
    fn parse_quiet_short_flag() {
        let cli = Cli::try_parse_from(["dot", "-q", "sync"]).unwrap();
        assert!(cli.quiet);
    }

    #[test]
    fn parse_verbose_short_flag() {
        let cli = Cli::try_parse_from(["dot", "-v", "sync"]).unwrap();
        assert!(cli.verbose);
    }
}
