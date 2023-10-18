/// Modules handles exports of commands in module folder
pub mod modules;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{str::SplitWhitespace, sync::Arc, collections::HashMap};

use crate::modules as modules_folder;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Doesn't Exist")]
    Invalid,
}

/// Command is the default template for command modules
/// Sub commands are EXPECTED to be handled by the run fn
/// help fn expects module to print it's own help message (default help message functions will be provided soon)
/// name fn is simply for indexing purposes (should return name of command)
pub trait Command {
    fn run(&self, args: SplitWhitespace) -> Result<(), Box<dyn Error>>;
    fn help(&self);
    fn name(&self) -> String;
}

// Static items that are can be compiled
lazy_static! {
    /// List of commands supported
    static ref COMMANDS_SET: Arc<Mutex<Vec<Box<dyn Command + Send + Sync>>>> = {
        let mut temp_set: Vec<Box<dyn Command + Send + Sync>> = vec![];

        // If user wants GUI features
        #[cfg(feature = "gui")]
        temp_set.append(&mut modules_folder::gui::add_commands());
        // debug stuff
        #[cfg(feature = "debug")]
        temp_set.append(&mut modules_folder::misc::add_commands());
        // network handlers
        temp_set.append(&mut modules_folder::handlers::add_commands());



        Arc::new(Mutex::new(temp_set))
    };
}

/// Intended for CLI
pub fn run_command(command: &str, args: SplitWhitespace) -> Result<(), Box<dyn Error>> {
    let cmd_guard = COMMANDS_SET.lock();
    if let Some(cmd) = cmd_guard.iter().find(|&cmd| cmd.name() == command) {
        return cmd.run(args);
    }

    // Hits if no commands are it
    return Err(ModuleError::Invalid.into());
}

fn parse_flags(input: SplitWhitespace) -> HashMap<String, String> {
    let mut flags_with_args = HashMap::new();
    let mut current_flag = String::new();

    for word in input {
        if word.starts_with('-') {
            if !current_flag.is_empty() {
                flags_with_args.insert(current_flag.clone(), String::new());
            }
            current_flag = word.trim_start_matches('-').to_owned();
        } else if !current_flag.is_empty() {
            flags_with_args.insert(current_flag.clone(), word.to_owned());
            current_flag.clear();
        }
    }

    if !current_flag.is_empty() {
        flags_with_args.insert(current_flag.clone(), String::new());
    }

    flags_with_args
}