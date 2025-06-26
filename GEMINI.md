# Gemini Project Context: dot

## Project Description

`dot` is a command-line tool for managing dotfiles. It simplifies the process of tracking and synchronizing configuration files across a user's system by maintaining a central repository of dotfiles and using symbolic links.

## Architecture

The project is a Rust-based command-line application with a modular structure:

-   **`main.rs`**: The application's entry point, responsible for initializing services and parsing command-line arguments.
-   **`cli.rs`**: Defines the command-line interface using the `clap` crate, mapping subcommands to their respective handlers.
-   **`commands/`**: A directory containing modules for each of the application's subcommands (`add`, `init`, `remove`, `sync`).
-   **`service.rs`**: Contains the core business logic for managing dotfiles.
-   **`fs/`**: A module that abstracts file system operations, such as creating symbolic links and moving files.
-   **`manifest.rs`**: Manages the `dot.toml` file, which tracks the dotfiles being managed by the tool.
-   **`error.rs`**: Defines custom error types for the application.
-   **`output.rs`**: Handles formatting and printing output to the console.
-   **`path_ext.rs`**: Provides utility functions for path manipulation.

## Dependencies

The project's dependencies are managed by Cargo and are listed in the `Cargo.toml` file. Key dependencies include:

-   **`clap`**: For parsing command-line arguments.
-   **`color-eyre`**: For pretty-printing error reports.
-   **`dirs`**: For resolving the user's home directory.
-   **`thiserror`**: For creating custom error types.
-   **`toml`**: For parsing and serializing the `dot.toml` manifest file.

## Build, Test, and Run

### Build

To build the project, run the following command:

```bash
cargo build
```

### Test

The project does not currently have a test suite. To add one, you can create a `tests` directory and add integration tests, or add unit tests within each module.

### Run

To run the application, use the following command:

```bash
cargo run -- [SUBCOMMAND] [OPTIONS]
```

For example, to initialize a new dotfiles repository, you would run:

```bash
cargo run -- init
```

### Install

To install the `dot` binary on your system, you can use the following command:

```bash
cargo install --path .
```

## Coding Style and Conventions

The project follows standard Rust conventions and formatting, as enforced by `rustfmt`. When contributing to the project, please ensure that your code is formatted using `rustfmt` before submitting a pull request.
