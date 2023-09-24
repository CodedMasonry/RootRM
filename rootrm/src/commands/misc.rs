use std::{str::SplitWhitespace, thread, time::Duration};

use anyhow::{Ok, Result};
use indicatif::ProgressIterator;

pub fn some_loading(mut args: SplitWhitespace) -> Result<()> {
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
