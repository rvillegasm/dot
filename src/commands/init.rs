use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{symlink::SymLinkOperations, FileSystem},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to initialize a new dot repository
pub struct InitCommand<
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
    InitCommand<'a, F, S, M, O>
{
    pub fn new(service: DotService<'a, F, S, M>, output: O) -> Self {
        Self { service, output }
    }
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> DotCommand
    for InitCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        self.service.init()?;
        self.output
            .display_success("Initialized empty dot repository");
        Ok(())
    }
}
