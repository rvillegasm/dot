mod cli;
mod commands;
mod error;
mod logger;
mod manifest;
mod path;

fn main() {
    if let Err(e) = cli::run() {
        log::error!("{e}");
        std::process::exit(1);
    }
}
