use std::fmt::Display;
use std::path::Path;

use crate::{
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::ManifestOperations,
    service::DotService,
};

/// A trait for commands that can provide user feedback
pub trait CommandOutput {
    fn display_success<D: Display>(&self, message: D);
}

/// Standard implementation that prints to stdout/stderr
#[derive(Clone)]
pub struct ConsoleOutput;

impl CommandOutput for ConsoleOutput {
    fn display_success<D: Display>(&self, message: D) {
        println!("{}", message);
    }
}

/// Command to initialize a new dot repository
pub struct InitCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> {
    service: DotService<F, S, M>,
    output: O,
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    InitCommand<F, S, M, O> 
{
    pub fn new(service: DotService<F, S, M>, output: O) -> Self {
        Self { service, output }
    }
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    crate::service::DotCommand for InitCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        self.service.init()?;
        self.output.display_success("Initialized empty dot repository");
        Ok(())
    }
}

/// Command to add a file to the dot repository
pub struct AddCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> {
    service: DotService<F, S, M>,
    output: O,
    file_path: String,
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    AddCommand<F, S, M, O>
{
    pub fn new(service: DotService<F, S, M>, output: O, file_path: String) -> Self {
        Self {
            service,
            output,
            file_path,
        }
    }
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    crate::service::DotCommand for AddCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        let original_path = Path::new(&self.file_path);
        let file_name = original_path
            .file_name()
            .ok_or_else(|| DotError::NotFound(original_path.to_path_buf()))?;
            
        let mut service = self.service.clone();
        service.add(&self.file_path)?;
        service.save_manifest()?;
        
        self.output.display_success(format!(
            "{} -> {}",
            file_name.to_string_lossy(),
            original_path.display()
        ));
        
        Ok(())
    }
}

/// Command to remove a file from the dot repository
pub struct RemoveCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> {
    service: DotService<F, S, M>,
    output: O,
    file_path: String,
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    RemoveCommand<F, S, M, O>
{
    pub fn new(service: DotService<F, S, M>, output: O, file_path: String) -> Self {
        Self {
            service,
            output,
            file_path,
        }
    }
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    crate::service::DotCommand for RemoveCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        let mut service = self.service.clone();
        service.remove(&self.file_path)?;
        service.save_manifest()?;
        
        self.output.display_success(format!(
            "Removed {}",
            Path::new(&self.file_path).display()
        ));
        
        Ok(())
    }
}

/// Command to sync all tracked files
pub struct SyncCommand<F: FileSystem, S: SymLinkOperations, M: ManifestOperations, O: CommandOutput> {
    service: DotService<F, S, M>,
    output: O,
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    SyncCommand<F, S, M, O>
{
    pub fn new(service: DotService<F, S, M>, output: O) -> Self {
        Self { service, output }
    }
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone, O: CommandOutput + Clone> 
    crate::service::DotCommand for SyncCommand<F, S, M, O>
{
    fn execute(&self) -> Result<(), DotError> {
        self.service.sync()?;
        
        if self.service.is_up_to_date() {
            self.output.display_success("Up to date");
        }
        
        Ok(())
    }
}
