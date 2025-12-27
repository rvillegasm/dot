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
pub struct AddMultipleCommand<
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
    AddMultipleCommand<'a, F, S, M, O>
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
    for AddMultipleCommand<'a, F, S, M, O>
{
    fn execute(&mut self) -> Result<(), DotError> {
        for path in &self.file_paths {
            let original_path = Path::new(path);
            let file_name = original_path
                .file_name()
                .ok_or_else(|| DotError::NotFound(original_path.to_path_buf()))?;

            self.service.add(path)?;

            self.output.display_success(format!(
                "{} -> {}",
                file_name.to_string_lossy(),
                original_path.display()
            ));
        }
        Ok(())
    }
}
