pub mod commands;

use std::str::SplitWhitespace;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Doesn't Exist")]
    Invalid,
}

pub fn run_command(command: &str, args: SplitWhitespace) -> Result<()> {
    match command {
        "load" => commands::misc::some_loading(args),
        _ => Err(ModuleError::Invalid.into()),
    }
}
