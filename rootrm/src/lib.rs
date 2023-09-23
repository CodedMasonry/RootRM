pub mod commands;

use anyhow::Result;

pub fn run_command(command: &str) -> Result<()> {
    match command {
        "load" => commands::misc::some_loading(args)
    }
}