use std::{env, path::Path, process::Command, str::SplitWhitespace};

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};
use tracing_subscriber;

/// Initialize tracing
fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();
    let code = {
        if let Err(e) = run() {
            eprintln!("ERROR: {e}");
            1
        } else {
            0
        }
    };
    ::std::process::exit(code);
}

/// Main handling code
#[tokio::main]
async fn run() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("RootRM > ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str())?;
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

                    // Attempts to run internal command; if internal command doesn't exist
                    // then attempts to run external command
                    command => match rootrm::run_command(command, args.clone()).await {
                        Ok(_) => continue,
                        Err(e) => {
                            if e.is::<rootrm::ModuleError>() {
                                run_external_command(command, args);
                            } else {
                                eprintln!("Error running command: {:#?}", e);
                            }
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

/// handle calling external functions if internal doesn't exist
fn run_external_command(command: &str, args: SplitWhitespace) {
    let child = Command::new(command).args(args).spawn();

    match child {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => eprintln!("{}", e),
    };
}
