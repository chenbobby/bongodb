use std::os::unix::net::UnixStream;
use std::io::prelude::*;

const UNIX_SOCKET_PATH: &str = "/tmp/bongodb.sock";

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH)?;
    stream.write_all(b"Hello, BongoDB.")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    Ok(())
}
