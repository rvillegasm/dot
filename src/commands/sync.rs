use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{symlink::SymLinkOperations, FileSystem},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to sync all tracked files
pub struct SyncCommand<
    'a,
    F: FileSystem,
    S: SymLinkOperations,
    M: ManifestOperations,
    O: CommandOutput,
> {
    service: DotService<'a, F, S, M>,
    output: O,
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput>
    SyncCommand<'a, F, S, M, O>
{
    pub fn new(service: DotService<'a, F, S, M>, output: O) -> Self {
        Self { service, output }
    }
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> DotCommand
    for SyncCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        self.service.sync()?;

        if self.service.is_up_to_date()? {
            self.output.display_success("Up to date");
        }

        Ok(())
    }
}
