mod cli;
mod commands;
mod error;
mod manifest;
mod path;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("\x1b[31mError: {e}\x1b[0m");
        std::process::exit(1);
    }
}
