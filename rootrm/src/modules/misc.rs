#![cfg(feature = "debug")]

use std::{str::SplitWhitespace, thread, time::Duration};
use indicatif::ProgressIterator;
use crate::Command;

pub struct TestCmd;
pub struct TestArgs;

pub fn add_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![
        Box::new(TestCmd),
        Box::new(TestArgs),
    ]
}

/// Debug CLI
impl crate::Command for TestCmd {
    fn run(&self, mut args: SplitWhitespace) -> Result<(), Box<(dyn std::error::Error)>> {
        let total: u32 = args.next().get_or_insert("100").parse()?;
        let mut result = 1;

        for i in (0..total).progress() {
            result += i;
            result = result / 3;

            thread::sleep(Duration::from_millis(1))
        }

        println!("{}", result);
        Ok(())
    }

    fn help(&self) {
        todo!()
    }

    fn name(&self) -> String {
        "load".to_string()
    }
}

impl crate::Command for TestArgs {
    fn run(&self, args: SplitWhitespace) -> Result<(), Box<(dyn std::error::Error)>> {
        println!("{:#?}", crate::parse_flags(args));
        Ok(())
    }

    fn help(&self) {
        todo!()
    }

    fn name(&self) -> String {
        "test_args".to_string()
    }
}