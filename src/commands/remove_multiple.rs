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
pub struct RemoveMultipleCommand<
    'a,
    F: FileSystem,
    S: SymLinkOperations,
    M: ManifestOperations,
    O: CommandOutput,
> {
    service: DotService<'a, F, S, M>,
    output: O,
    file_paths: Vec<String>,
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput>
    RemoveMultipleCommand<'a, F, S, M, O>
{
    pub fn new(service: DotService<'a, F, S, M>, output: O, file_paths: Vec<String>) -> Self {
        Self {
            service,
            output,
            file_paths,
        }
    }
}

impl<'a, F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> DotCommand
    for RemoveMultipleCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        for path in &self.file_paths {
            self.service.remove(path)?;

            self.output
                .display_success(format!("Removed {}", Path::new(path).display()));
        }

        Ok(())
    }
}
