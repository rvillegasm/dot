use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{symlink::SymLinkOperations, FileSystem},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to sync all tracked files
pub struct SyncCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput>
{
    service: DotService<F, S, M>,
    output: O,
}

impl<
        F: FileSystem + Clone,
        S: SymLinkOperations + Clone,
        M: ManifestOperations + Clone,
        O: CommandOutput + Clone,
    > SyncCommand<F, S, M, O>
{
    pub fn new(service: DotService<F, S, M>, output: O) -> Self {
        Self { service, output }
    }
}

impl<
        F: FileSystem + Clone,
        S: SymLinkOperations + Clone,
        M: ManifestOperations + Clone,
        O: CommandOutput + Clone,
    > DotCommand for SyncCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        self.service.sync()?;

        if self.service.is_up_to_date()? {
            self.output.display_success("Up to date");
        }

        Ok(())
    }
}
