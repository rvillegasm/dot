use std::path::Path;

use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{symlink::SymLinkOperations, FileSystem},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to add a file to the dot repository
pub struct AddCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput>
{
    service: DotService<F, S, M>,
    output: O,
    file_path: String,
}

impl<
        F: FileSystem + Clone,
        S: SymLinkOperations + Clone,
        M: ManifestOperations + Clone,
        O: CommandOutput + Clone,
    > AddCommand<F, S, M, O>
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
    > DotCommand for AddCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        let original_path = Path::new(&self.file_path);
        let file_name = original_path
            .file_name()
            .ok_or_else(|| DotError::NotFound(original_path.to_path_buf()))?;

        let mut service = self.service.clone();
        service.add(&self.file_path)?;

        self.output.display_success(format!(
            "{} -> {}",
            file_name.to_string_lossy(),
            original_path.display()
        ));

        Ok(())
    }
}
