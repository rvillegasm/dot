use std::path::Path;

use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to remove a file from the dot repository
pub struct RemoveCommand<
    'a,
    F: FileSystem,
    S: SymLinkOperations,
    M: ManifestOperations,
    O: CommandOutput,
> {
    service: DotService<'a, F, S, M>,
    output: O,
    file_path: String,
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput>
    RemoveCommand<'a, F, S, M, O>
{
    pub fn new(service: DotService<'a, F, S, M>, output: O, file_path: String) -> Self {
        Self {
            service,
            output,
            file_path,
        }
    }
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> DotCommand
    for RemoveCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        self.service.remove(&self.file_path)?;

        self.output
            .display_success(format!("Removed {}", Path::new(&self.file_path).display()));

        Ok(())
    }
}
