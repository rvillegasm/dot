# DOT - A Dotfile Manager

A simple and elegant dotfile manager for Unix-like systems, refactored to follow SOLID principles and idiomatic Rust conventions.

## Architecture Overview

The codebase follows a domain-driven design with a clean separation of concerns:

### Core Components

1. **File System Operations**
   - `FileSystem` trait: Defines operations for file system interactions
   - `StdFileSystem`: Implementation using standard library

2. **Symbolic Link Handling**
   - `SymLinkOperations` trait: Defines operations for managing symlinks
   - `UnixSymLinkOperations`: Unix-specific implementation

3. **Manifest Management**
   - `ManifestOperations` trait: Defines operations for tracking dotfiles
   - `Manifest`: Implementation storing file mappings in TOML format

4. **Business Logic**
   - `DotService`: Core service implementing dotfile management functionality
   - `DotCommand`: Generic interface for all commands

5. **CLI Interface**
   - `Commands`: Implementation of each CLI command
   - `CommandOutput`: Interface for user feedback

### SOLID Principles Applied

1. **Single Responsibility Principle (SRP)**
   - Each module has a clear, well-defined responsibility
   - File operations separated from symlink operations
   - Core logic separated from user interface

2. **Open/Closed Principle (OCP)**
   - Interfaces allow extension without modification
   - New file system or symlink handling can be added without changing core logic

3. **Liskov Substitution Principle (LSP)**
   - Implementations can be swapped without affecting behavior
   - Example: Could add Windows symlink handling without changing other components

4. **Interface Segregation Principle (ISP)**
   - Small, focused interfaces with minimal methods
   - Commands only need to implement execute()

5. **Dependency Inversion Principle (DIP)**
   - High-level components depend on abstractions
   - Service takes FileSystem, SymLinkOperations, and ManifestOperations traits

### Idiomatic Rust Features

- Extensive use of traits for abstractions
- Proper error handling with thiserror
- Clone derivation for multi-threaded support
- Path handling using Rust's standard library
- Generic implementation with constraints

## Usage

```bash
# Initialize a new dot repository
dot init

# Add a file to track
dot add ~/.vimrc

# Sync all tracked files
dot sync

# Remove a tracked file
dot remove .vimrc
```

## Testing

The codebase includes unit tests for core components.
