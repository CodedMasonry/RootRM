use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, help = "Removes interactive elements")]
    no_interact: bool,
}

fn main() -> Result<()> {
    let env_stff: Args = Args::parse();

    loop {
        print!("RootRM > ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }

            "exit" => return Ok(()),

            command => match rootrm::run_command(command) {
                Ok(_) => continue,
                Err(_) => {
                    let child = Command::new(command).args(args).spawn();

                    match child {
                        Ok(mut child) => {
                            child.wait()?;
                        }
                        Err(e) => eprintln!("{}", e),
                    };
                }
            },
        }
    }
}
