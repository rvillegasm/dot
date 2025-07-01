use std::path::Path;

use crate::{
    commands::DotCommand,
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::ManifestOperations,
    output::CommandOutput,
    service::DotService,
};

/// Command to add a file to the dot repository
pub struct AddCommand<
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
    AddCommand<'a, F, S, M, O>
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
    for AddCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        let original_path = Path::new(&self.file_path);
        let file_name = original_path
            .file_name()
            .ok_or_else(|| DotError::NotFound(original_path.to_path_buf()))?;

        self.service.add(&self.file_path)?;

        self.output.display_success(format!(
            "{} -> {}",
            file_name.to_string_lossy(),
            original_path.display()
        ));

        Ok(())
    }
}
