mod add;
mod init;
mod remove;
mod sync;

pub use add::AddCommand;
pub use init::InitCommand;
pub use remove::RemoveCommand;
pub use sync::SyncCommand;

use crate::error::Result;

/// Trait for executable commands.
/// Each command is self-contained and directly uses the abstractions it needs.
pub trait Command {
    /// Execute the command
    fn execute(self) -> Result<()>;
}
