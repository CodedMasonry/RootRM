pub mod modules;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{str::SplitWhitespace, sync::Arc};

use crate::modules as modules_folder;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Doesn't Exist")]
    Invalid,
}

pub trait Command {
    fn run(&self, args: SplitWhitespace) -> Result<()>;
    fn help(&self);
    fn name(&self) -> String;
}

lazy_static! {
    static ref COMMANDS_SET: Arc<Mutex<Vec<Box<dyn Command + Send + Sync>>>> =
        Arc::new(Mutex::new(vec![Box::new(modules_folder::misc::TestCmd()),]));
}

pub fn run_command(command: &str, args: SplitWhitespace) -> Result<()> {
    let cmd_guard = COMMANDS_SET.lock();
    if let Some(cmd) = cmd_guard.iter().find(|&cmd| cmd.name() == command) {
        println!("FOUND");

        return cmd.run(args);
    }

    // Hits if no commands are it
    return Err(ModuleError::Invalid.into());
}
