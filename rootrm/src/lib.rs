/// Modules handles exports of commands in module folder
pub mod modules;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use async_trait::async_trait;
use anyhow::Result;
use std::{collections::HashMap, str::SplitWhitespace, sync::Arc};

use crate::modules as modules_folder;
use thiserror::Error;

/// Basic error handling for root module handling
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Command Doesn't Exist")]
    NonExistant,
}

/// Command is the default template for command modules
/// Sub commands are EXPECTED to be handled by the run fn
/// help fn expects module to print it's own help message (default help message functions will be provided soon)
/// name fn is simply for indexing purposes (should return name of command)
#[async_trait]
pub trait Command: Send + Sync {
    async fn run(&self, args: SplitWhitespace<'_>) -> Result<()>;
    fn help(&self);
    fn name(&self) -> String;
}

// Static items that are can be compiled
lazy_static! {
    /// List of commands supported
    /// Append module additions to this set
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

/// Intended for CLI; attempts to run cmd
pub async fn run_command(command: &str, args: SplitWhitespace<'_>) -> Result<()> {
    let cmd_guard = COMMANDS_SET.lock();
    if let Some(cmd) = cmd_guard.iter().find(|&cmd| cmd.name() == command) {
        return cmd.run(args).await;
    }

    // Hits if no commands are it
    return Err(ModuleError::NonExistant.into());
}

/// Handles parsing flags in a SplitWhitespace item
/// I know it isn't clean but it works
async fn parse_flags(input: SplitWhitespace<'_>) -> HashMap<String, String> {
    let mut flags_with_args = HashMap::new();
    let mut current_flag = String::new();
    let mut is_long_string = false;
    let mut long_string = Vec::new(); // In case someone has a long input ("my home/repos")

    for word in input {
        if word.starts_with('-') {
            if !current_flag.is_empty() {
                flags_with_args.insert(current_flag.clone(), String::new());
            }
            current_flag = word.trim_start_matches('-').to_owned();
        } else if !current_flag.is_empty() {
            if word.starts_with("\"") {
                long_string.push(word.trim_start_matches('\"'));
                is_long_string = true
            } else if word.ends_with("\"") {
                long_string.push(word.trim_end_matches('\"'));

                flags_with_args.insert(current_flag.clone(), long_string.join(" "));
                long_string.clear();
                current_flag.clear();

                is_long_string = false;
            } else if is_long_string == true {
                long_string.push(word);
            } else {
                flags_with_args.insert(current_flag.clone(), word.to_owned());
                current_flag.clear();
            }
        }
    }

    if !current_flag.is_empty() {
        flags_with_args.insert(current_flag.clone(), String::new());
    }

    flags_with_args
}
