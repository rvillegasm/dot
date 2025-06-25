mod add;
mod init;
mod remove;
mod sync;

use crate::error::DotError;

pub use add::AddCommand;
pub use init::InitCommand;
pub use remove::RemoveCommand;
pub use sync::SyncCommand;

/// A trait defining the core operations for the dot application
pub trait DotCommand {
    /// Execute the command
    fn execute(&self) -> Result<(), DotError>;
}
