# Dot - A simple Dotfiles Manager

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)
![Crates.io](https://img.shields.io/crates/v/dot_rust.svg)
![GitHub stars](https://img.shields.io/github/stars/rvillegasm/dot.svg)
![GitHub issues](https://img.shields.io/github/issues/rvillegasm/dot.svg)

A simple and elegant dotfile manager for Unix-like systems.

`dot` helps you manage your configuration files (dotfiles) by creating symbolic links from a centralized repository to their intended locations in your home directory (or wherever you want).

## Features

- **Initialize:** Create a new dotfiles repository.
- **Add:** Start tracking a new dotfile.
- **Remove:** Stop tracking a dotfile.
- **Sync:** Synchronize all your dotfiles, creating symbolic links.

## Installation

### From Crates.io

You can install `dot` using `cargo`:

```bash
cargo install dot_rust
```

### Build from Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/rvillegasm/dot.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd dot
    ```
3.  Build the project:
    ```bash
    cargo build --release
    ```
4.  The binary will be located at `target/release/dot`. You can move it to a directory in your `$PATH`.

## Usage

```bash
# Initialize a new dot repository (defaults to ~/.dotfiles)
dot init

# Add a file to track
dot add ~/.vimrc

# Sync all tracked files
dot sync

# Remove a tracked file
dot remove .vimrc
```

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request if you have any ideas, suggestions, or bug reports.

### Code of Conduct

We adhere to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). All contributors are expected to uphold this code.

## License

This project is licensed under the MIT License. See the [LICENSE.txt](LICENSE.txt) file for details.

## Contact

Rafael Villegas - [@rvillegasm](https://github.com/rvillegasm) - rafa.villegas.michel@gmail.com
