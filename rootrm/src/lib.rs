pub mod modules;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{str::SplitWhitespace, sync::Arc};

use crate::modules as modules_folder;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Doesn't Exist")]
    Invalid,
}

pub trait Command {
    fn run(&self, args: SplitWhitespace) -> Result<(), Box<dyn Error>>;
    fn help(&self);
    fn name(&self) -> String;
}

// Static items that are can be compiled
lazy_static! {
    static ref COMMANDS_SET: Arc<Mutex<Vec<Box<dyn Command + Send + Sync>>>> = {

        // Default commands
        let mut temp_set: Vec<Box<dyn Command + Send + Sync>> = vec![
            Box::new(modules_folder::misc::TestCmd),
        ];

        // If user wants GUI features, add it
        #[cfg(feature = "gui")]
        temp_set.append(&mut modules_folder::gui::add_commands());


        Arc::new(Mutex::new(temp_set))
    };
}
pub fn run_command(command: &str, args: SplitWhitespace) -> Result<(), Box<dyn Error>> {
    let cmd_guard = COMMANDS_SET.lock();
    if let Some(cmd) = cmd_guard.iter().find(|&cmd| cmd.name() == command) {
        return cmd.run(args);
    }

    // Hits if no commands are it
    return Err(ModuleError::Invalid.into());
}
