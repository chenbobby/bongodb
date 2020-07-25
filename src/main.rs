use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::{UnixStream, UnixListener};
use std::thread;

const UNIX_SOCKET_PATH: &str = "/tmp/bongodb.sock";

fn main() {
    // Initialize and bind a Unix socket.
    let socket =
        match UnixListener::bind(UNIX_SOCKET_PATH) {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Failed to bind socket: {}", e);
                std::process::exit(1);
            }
        };

    // Begin listening for incoming connections.
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            },
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn handle_client(stream: UnixStream) {
    let stream_reader = BufReader::new(&stream);
    let mut stream_writer = BufWriter::new(&stream);

    for line in stream_reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
                stream_writer.write(&line.into_bytes()).unwrap();
                stream_writer.flush().unwrap();
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }
}
