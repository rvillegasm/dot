use std::fmt::Display;

/// A trait for commands that can provide user feedback
pub trait CommandOutput {
    fn display_success<D: Display>(&self, message: D);
}

/// Standard implementation that prints to stdout/stderr
pub struct ConsoleOutput;

impl CommandOutput for ConsoleOutput {
    fn display_success<D: Display>(&self, message: D) {
        println!("{}", message);
    }
}
