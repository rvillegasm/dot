use std::path::Path;

use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{symlink::SymLinkOperations, FileSystem},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to remove a file from the dot repository
pub struct RemoveCommand<
    F: FileSystem,
    S: SymLinkOperations,
    M: ManifestOperations,
    O: CommandOutput,
> {
    service: DotService<F, S, M>,
    output: O,
    file_path: String,
}

impl<
        F: FileSystem + Clone,
        S: SymLinkOperations + Clone,
        M: ManifestOperations + Clone,
        O: CommandOutput + Clone,
    > RemoveCommand<F, S, M, O>
{
    pub fn new(service: DotService<F, S, M>, output: O, file_path: String) -> Self {
        Self {
            service,
            output,
            file_path,
        }
    }
}

impl<
        F: FileSystem + Clone,
        S: SymLinkOperations + Clone,
        M: ManifestOperations + Clone,
        O: CommandOutput + Clone,
    > DotCommand for RemoveCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        let mut service = self.service.clone();
        service.remove(&self.file_path)?;

        self.output
            .display_success(format!("Removed {}", Path::new(&self.file_path).display()));

        Ok(())
    }
}
