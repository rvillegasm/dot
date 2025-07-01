mod cli;
mod commands;
mod error;
mod fs;
mod manifest;
mod output;
mod path_ext;
mod service;

fn main() {
    if let Err(e) = cli::parse() {
        eprintln!("\x1b[31mError: {e}\x1b[0m");
        std::process::exit(1);
    }
}
