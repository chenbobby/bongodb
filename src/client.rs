use std::os::unix::net::UnixStream;
use std::io::prelude::*;

const UNIX_SOCKET_PATH: &str = "/tmp/bongodb.sock";

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH)?;
    stream.write_all(b"Hello, BongoDB.\n")?;
    let mut buffer = [0; 64];
    stream.read(&mut buffer)?;
    let data = String::from_utf8(buffer.to_vec()).unwrap();
    println!("{}", &data);
    Ok(())
}
