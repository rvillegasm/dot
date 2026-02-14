use env_logger::{Builder, Env};

mod cli;
mod commands;
mod error;
mod manifest;
mod path;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    if let Err(e) = cli::parse() {
        log::error!("\x1b[31m{e}\x1b[0m");
        std::process::exit(1);
    }
}
