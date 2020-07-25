use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::io;
use std::io::prelude::*;

const UNIX_SOCKET_PATH: &str = "/tmp/bongodb.sock";

fn main() -> std::io::Result<()> {
    let stdin_reader = BufReader::new(io::stdin());
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH)?;
    let mut buffer = [0; 64];

    for line in stdin_reader.lines() {
        match line {
            Ok(line) => {
                println!("Read: {:?}", line);

                writeln!(stream, "{}", line).unwrap();
                println!("Written.");

                stream.read(&mut buffer)?;
                let data = String::from_utf8(buffer.to_vec()).unwrap();
                println!("Received: {}", &data);
            },
            Err(err) => {
                println!("{}", err);
                break;
            },
        }
    }
    Ok(())
}
