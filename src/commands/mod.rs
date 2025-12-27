mod add;
mod add_multiple;
mod init;
mod remove;
mod remove_multiple;
mod sync;

use crate::error::DotError;

pub use add::AddCommand;
pub use add_multiple::AddMultipleCommand;
pub use init::InitCommand;
pub use remove::RemoveCommand;
pub use remove_multiple::RemoveMultipleCommand;
pub use sync::SyncCommand;

/// A trait defining the core operations for the dot application
pub trait DotCommand {
    /// Execute the command
    fn execute(&mut self) -> Result<(), DotError>;
}
