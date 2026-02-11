<div align="center">
  <img src="assets/logo.png" alt="dot logo" width="200"/>
</div>

# Dot - A Simple Dotfiles Manager

![Build Status](https://github.com/rvillegasm/dot/workflows/Build%20and%20Test/badge.svg)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)
![Crates.io](https://img.shields.io/crates/v/dot_rust.svg)
![Crates.io Downloads](https://img.shields.io/crates/d/dot_rust.svg)
![GitHub stars](https://img.shields.io/github/stars/rvillegasm/dot.svg)
![GitHub issues](https://img.shields.io/github/issues/rvillegasm/dot.svg)

A simple and elegant dotfile manager for Unix-like systems. Dotfiles are configuration files (like `.vimrc`, `.zshrc`) that customize your system. `dot` helps you manage these files by creating symbolic links from a centralized repository to their intended locations, making it easy to version control and sync your configurations across multiple machines.

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Basic Commands](#basic-commands)
  - [Command Reference](#command-reference)
  - [The Manifest File](#the-manifest-file)
- [Configuration](#configuration)
- [Development](#development)
  - [Building](#building)
  - [Running Tests](#running-tests)
  - [Contributing](#contributing-1)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)
- [Changelog](#changelog)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Quick Start

Get up and running with `dot` in just a few minutes:

```bash
# Install dot
cargo install dot_rust

# Initialize in your dotfiles directory
cd ~/dotfiles
dot init

# Start tracking your vim config
dot add ~/.vimrc

# On a new machine, clone your dotfiles repo and sync
git clone <your-dotfiles-repo> ~/dotfiles
cd ~/dotfiles
dot sync
```

## Features

- **Initialize:** Create a new dotfiles repository with manifest tracking.
- **Add:** Start tracking a new dotfile by moving it to your repository and creating a symlink.
- **Remove:** Stop tracking a dotfile, restoring it to its original location.
- **Sync:** Synchronize all your dotfiles, creating symbolic links for tracked files.
- **Portable:** Uses tilde (`~`) expansion for paths, making your dotfiles portable across machines.
- **Lightweight:** Simple, fast, and minimal dependencies.

## Installation

### Prerequisites

- **Rust 1.70+** (for building from source or installing via cargo)
- **Unix-like OS:** Linux, macOS, BSD, or WSL on Windows

### From Crates.io

You can install `dot` using `cargo`:

```bash
cargo install dot_rust
```

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/leadingperiod/dot.git
   ```
2. Navigate to the project directory:
   ```bash
   cd dot
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be located at `target/release/dot`. You can move it to a directory in your `$PATH`.

## Usage

### Basic Commands

```bash
# Initialize a new dot repository (creates dot.toml in current directory)
dot init

# Add a file to track
dot add ~/.vimrc

# Add a config directory file
dot add ~/.config/nvim/init.vim

# Sync all tracked files (creates symlinks)
dot sync

# Remove a tracked file (restores original, removes symlink)
dot remove .vimrc
```

### Command Reference

- **`dot init`** - Creates a `dot.toml` manifest file in the current directory. This initializes your dotfiles repository.

- **`dot add <path>`** - Starts tracking a file:

  - Moves the file from its original location to your dotfiles repository
  - Creates a symbolic link at the original location pointing to the repository copy
  - Adds an entry to `dot.toml` mapping the filename to its original path

- **`dot remove <filename>`** - Stops tracking a file:

  - Removes the symbolic link at the original location
  - Moves the file back to its original location
  - Removes the entry from `dot.toml`

- **`dot sync`** - Synchronizes your dotfiles:
  - Reads all entries from `dot.toml`
  - Creates symbolic links for any tracked files that don't have them
  - Useful when setting up dotfiles on a new machine

### The Manifest File

`dot` uses a TOML manifest file (`dot.toml`) to track your dotfiles. This file maps local filenames to their original paths:

```toml
# dot.toml
".vimrc" = "~/.vimrc"
".zshrc" = "~/.zshrc"
"config/nvim/init.vim" = "~/.config/nvim/init.vim"
```

The manifest stores paths with tilde (`~`) prefixes for portability across different systems and user accounts.

## Configuration

`dot` keeps things simple:

- **Manifest Location:** The `dot.toml` file is stored in your dotfiles repository root (wherever you ran `dot init`)
- **File Storage:** Tracked files are stored in the same directory as the manifest, maintaining their relative paths
- **Symlinks:** Original file locations contain symbolic links pointing back to your repository
- **Portability:** Paths use `~` expansion, so your dotfiles work across different machines and users

## Development

### Building

```bash
cargo build                  # Build debug version
cargo build --release        # Build optimized release version
```

### Running Tests

```bash
cargo test                   # Run all tests (unit + integration)
cargo test --lib             # Run unit tests only
cargo test --test integration # Run integration tests only
cargo clippy                 # Run linter
cargo fmt                    # Format code
```

The project follows a test-driven approach with:

- **Unit tests:** Located in each module under `#[cfg(test)] mod tests`
- **Integration tests:** Located in `tests/integration.rs` using `tempfile::TempDir` for isolation

### Contributing

For detailed architecture and development conventions, see [CLAUDE.md](CLAUDE.md). Key points:

- **Commit Format:** Use [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `refactor:`, `test:`, `chore:`)
- **Code Style:** Run `cargo fmt` and `cargo clippy` before committing
- **Testing:** Add tests for new features and ensure all tests pass

## Troubleshooting

### Common Issues

**"File already exists" when adding a file**

- The file may already be tracked in `dot.toml`. Check your manifest or use a different name.

**"Not a symlink" when removing a file**

- The file at the original location was modified or replaced outside of `dot`. You may need to manually resolve this by checking `dot.toml` and the repository.

**Permission errors**

- Ensure you have write access to both the dotfiles repository and the target locations (usually your home directory).

**Symlink points to wrong location**

- Check that you're running `dot sync` from the correct directory (your dotfiles repository root where `dot.toml` lives).

### Manual Recovery

If something goes wrong, remember:

- Your files are safely stored in your dotfiles repository directory
- Check `dot.toml` to see what's tracked and where files should link
- You can manually create symlinks if needed: `ln -s /path/to/repo/.vimrc ~/.vimrc`

## FAQ

**Q: What are dotfiles?**
A: Dotfiles are configuration files in Unix-like systems, typically starting with a dot (`.`) like `.bashrc`, `.vimrc`, or `.gitconfig`. They customize your shell, editor, and other tools.

**Q: Where should I initialize my dot repository?**
A: Anywhere you like! Common choices are `~/dotfiles`, `~/.dotfiles`, or a directory you sync with Git. Just `cd` there and run `dot init`.

**Q: Can I track files outside my home directory?**
A: Yes! `dot add /etc/someconfig` works, though you may need appropriate permissions.

**Q: What happens if I delete a symlink?**
A: The symlink will be gone but your file remains safe in the repository. Run `dot sync` to recreate the symlink.

**Q: How do I sync to a new machine?**
A: Clone your dotfiles repository (e.g., from GitHub), navigate to it, and run `dot sync`. All symlinks will be created.

**Q: Can I use this with Git?**
A: Absolutely! That's the recommended workflow. Initialize your dotfiles directory as a Git repo, track files with `dot`, then commit and push to GitHub/GitLab for backup and sync across machines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each version.

View releases on [GitHub Releases](https://github.com/leadingperiod/dot/releases).

## Contributing

We welcome contributions! Here's how you can help:

**Types of Contributions:**

- **Bug Reports:** Open an issue describing the problem and steps to reproduce
- **Feature Requests:** Suggest new features or improvements
- **Documentation:** Help improve docs, add examples, or fix typos
- **Code:** Submit pull requests for bug fixes or new features

**Pull Request Process:**

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes and add tests
4. Ensure all tests pass (`cargo test`)
5. Commit using conventional commit format (`feat: add amazing feature`)
6. Push to your fork and open a pull request

### Code of Conduct

We adhere to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). All contributors are expected to uphold this code.

## License

This project is licensed under the MIT License. See the [LICENSE.txt](LICENSE.txt) file for details.

## Contact

Rafael Villegas - [@rvillegasm](https://github.com/rvillegasm) - rafa.villegas.michel@gmail.com
