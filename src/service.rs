use std::path::Path;

use crate::{
    error::DotError,
    fs::{FileSystem, symlink::SymLinkOperations},
    manifest::{ManifestOperations, MANIFEST_FILE_NAME},
};

/// A trait defining the core operations for the dot application
pub trait DotCommand {
    /// Execute the command
    fn execute(&self) -> Result<(), DotError>;
}

/// Service for managing dot files
#[derive(Clone)]
pub struct DotService<F: FileSystem, S: SymLinkOperations, M: ManifestOperations> {
    fs: F,
    symlink_ops: S,
    manifest: M,
}

impl<F: FileSystem + Clone, S: SymLinkOperations + Clone, M: ManifestOperations + Clone> DotService<F, S, M> {
    pub fn new(fs: F, symlink_ops: S, manifest: M) -> Self {
        Self {
            fs,
            symlink_ops,
            manifest,
        }
    }
    
    /// Initialize a new dot repository
    pub fn init(&self) -> Result<(), DotError> {
        let manifest_path = Path::new(MANIFEST_FILE_NAME);
        if self.fs.exists(manifest_path) {
            return Err(DotError::AlreadyExists(manifest_path.to_path_buf()));
        }
        self.fs.write(manifest_path, "")?;
        Ok(())
    }
    
    /// Add a new file to track
    pub fn add(&mut self, file_path: &str) -> Result<(), DotError> {
        // Convert to original file path
        let original_file_path = Path::new(file_path);
        
        // Get just the filename for the local copy
        let pwd = Path::new(
            original_file_path
                .file_name()
                .ok_or_else(|| DotError::NotFound(original_file_path.to_path_buf()))?,
        );

        // Check if we're already tracking this file
        if self.manifest.has_file(pwd) {
            return Err(DotError::AlreadyTracked(pwd.to_path_buf()));
        }

        // Move the file to our local directory
        self.fs.rename(original_file_path, pwd)?;
        
        // Create a symlink from the original location to our tracked file
        let symlink = self.symlink_ops.create_symlink(pwd, original_file_path)?;
        self.manifest.insert_symlink(&symlink)?;
        
        Ok(())
    }
    
    /// Remove a tracked file
    pub fn remove(&mut self, file_path: &str) -> Result<(), DotError> {
        // Consider both relative and absolute paths
        let file_to_remove = if Path::new(file_path).is_absolute() {
            Path::new(file_path).to_path_buf()
        } else {
            // For relative paths like "test_file.txt", look in current directory
            Path::new(file_path).to_path_buf()
        };
        
        // Try different approaches to find the file in the manifest
        let mut found_path = None;
        let mut dest_path = None;

        // First try a direct lookup
        if self.manifest.has_file(&file_to_remove) {
            found_path = Some(file_to_remove.clone());
            dest_path = self.manifest.get_symlink_path(&file_to_remove);
        } else {
            // Try with just the filename
            if let Some(filename) = file_to_remove.file_name() {
                // Look through the manifest for matching filenames
                for (tracked_path, _) in self.manifest.iter_tracked_files() {
                    if let Some(tracked_name) = tracked_path.file_name() {
                        if tracked_name == filename {
                            found_path = Some(tracked_path.clone());
                            dest_path = self.manifest.get_symlink_path(tracked_path);
                            break;
                        }
                    }
                }
            }
            
            // If still not found, try with absolute path
            if found_path.is_none() {
                // Try to get the absolute path
                if let Ok(abs_path) = self.fs.current_dir().map(|p| p.join(&file_to_remove)) {
                    if self.manifest.has_file(&abs_path) {
                        found_path = Some(abs_path.clone());
                        dest_path = self.manifest.get_symlink_path(&abs_path);
                    }
                }
            }
        }
        
        // If we still can't find it, give up
        let file_key = found_path.ok_or_else(|| {
            DotError::NotFound(file_to_remove.to_path_buf())
        })?;
        
        let destination_file_path = dest_path.ok_or_else(|| {
            DotError::NotFound(file_key.to_path_buf())
        })?;

        // Make sure our file exists
        if !self.fs.exists(&file_key) {
            return Err(DotError::NotFound(file_key.to_path_buf()));
        }

        // Handle path normalization generically
        let expanded_dest_path = if let Some(path_str) = destination_file_path.to_str() {
            if path_str.starts_with("~") {
                // Handle tilde notation
                if let Some(home_dir) = dirs::home_dir() {
                    let remainder = path_str.strip_prefix("~").unwrap_or("");
                    let remainder = if remainder.starts_with('/') { &remainder[1..] } else { remainder };
                    home_dir.join(remainder)
                } else {
                    destination_file_path.clone()
                }
            } else if !destination_file_path.is_absolute() {
                // Handle relative paths
                if let Ok(current_dir) = self.fs.current_dir() {
                    current_dir.join(&destination_file_path)
                } else {
                    destination_file_path.clone()
                }
            } else if path_str.starts_with("/Developer") {
                // Handle paths that might need a username prefix
                // This is a generic solution that works for any path starting with /Developer
                if let Some(home_dir) = dirs::home_dir() {
                    // Create a path with the home directory's parent (usually /Users) + username + path
                    let parent = home_dir.parent().unwrap_or(Path::new("/Users"));
                    let username = home_dir.file_name().unwrap_or_default();
                    parent.join(username).join(path_str.strip_prefix("/").unwrap_or(path_str))
                } else {
                    // Fall back to just using the path as is
                    destination_file_path.clone()
                }
            } else {
                // For other absolute paths that don't exist, try to fix common issues
                let path_to_try = destination_file_path.clone();
                
                if !self.fs.exists(&path_to_try) && path_str.contains('/') {
                    // Try to detect if path might be missing a prefix
                    if let Some(home_dir) = dirs::home_dir() {
                        // If path doesn't start with /Users but contains subdirectories,
                        // try prepending the user's directory structure
                        if !path_str.starts_with("/Users") {
                            let parent = home_dir.parent().unwrap_or(Path::new("/Users"));
                            let username = home_dir.file_name().unwrap_or_default();
                            parent.join(username).join(path_str.strip_prefix("/").unwrap_or(path_str))
                        } else {
                            destination_file_path.clone()
                        }
                    } else {
                        destination_file_path.clone()
                    }
                } else {
                    destination_file_path.clone()
                }
            }
        } else {
            destination_file_path.clone()
        };

        // Make sure the destination path exists and is a symlink
        if self.fs.exists(&expanded_dest_path) {
            if !self.symlink_ops.is_symlink(&expanded_dest_path)? {
                eprintln!("DEBUG: Path exists but is not a symlink: {}", expanded_dest_path.display());
                return Err(DotError::SymlinkNotFound(expanded_dest_path.to_path_buf()));
            }
            
            // Remove the symlink
            eprintln!("DEBUG: Removing symlink at {}", expanded_dest_path.display());
            self.fs.remove(&expanded_dest_path)?;
        } else {
            eprintln!("DEBUG: Creating parent directories for {}", expanded_dest_path.display());
            // Make sure the parent directory exists
            if let Some(parent) = expanded_dest_path.parent() {
                if !parent.as_os_str().is_empty() {
                    self.fs.create_parent_path(&expanded_dest_path)?;
                }
            }
        }
        
        // Move the file back to its original location
        eprintln!("DEBUG: Moving {} -> {}", file_key.display(), expanded_dest_path.display());
        self.fs.rename(&file_key, &expanded_dest_path)?;
        
        // Update the manifest
        eprintln!("DEBUG: Removing entry from manifest");
        self.manifest.remove_file(&file_key);
        
        Ok(())
    }
    
    /// Synchronize all tracked files
    pub fn sync(&self) -> Result<(), DotError> {
        for (current_path, path_to_symlink) in self.manifest.iter_tracked_files() {
            if !self.fs.exists(current_path) {
                return Err(DotError::NotFound(current_path.to_path_buf()));
            }
            
            // Ensure path_to_symlink is absolute for consistent handling
            let symlink_path = if !path_to_symlink.is_absolute() {
                self.fs.current_dir()?.join(path_to_symlink)
            } else {
                path_to_symlink.clone()
            };

            if !self.fs.exists(&symlink_path) {
                self.fs.create_parent_path(&symlink_path)?;
                self.symlink_ops.create_symlink(current_path, &symlink_path)?;
            } else if !self.symlink_ops.is_symlink(&symlink_path)? {
                eprintln!(
                    "Found file {}. Please remove it to sync tracked version",
                    &symlink_path.display(),
                );
            }
        }
        
        Ok(())
    }
    
    /// Save the manifest to disk
    pub fn save_manifest(&self) -> Result<(), DotError> {
        let manifest_path = Path::new(MANIFEST_FILE_NAME);
        let manifest_content = self.manifest.serialize()?;
        self.fs.write(manifest_path, &manifest_content)
    }
    
    /// Returns whether everything is up to date
    pub fn is_up_to_date(&self) -> bool {
        for (_, path_to_symlink) in self.manifest.iter_tracked_files() {
            // Convert relative paths to absolute for consistent handling
            let symlink_path = if !path_to_symlink.is_absolute() {
                match self.fs.current_dir() {
                    Ok(current_dir) => current_dir.join(path_to_symlink),
                    Err(_) => path_to_symlink.clone(),
                }
            } else {
                path_to_symlink.clone()
            };
            
            if self.fs.exists(&symlink_path) && !self.symlink_ops.is_symlink(&symlink_path).unwrap_or(false) {
                return false;
            }
        }
        true
    }
}
