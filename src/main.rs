use std::{mem, thread};
use std::io::{Read, Write};
use std::os::unix::net::{UnixStream, UnixListener};

extern crate bongodb;
use bongodb::message::{MessageHeader, MessageStatus};

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
    println!("BongoDB server is now listening on Unix socket {}", UNIX_SOCKET_PATH);
    for stream in socket.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || handle_client(&mut stream));
            },
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn handle_client(stream: &mut UnixStream) {
    println!("New client connected.");
    loop {
        // Read in raw bytes from the Unix stream (for the message header).
        let mut in_header_bytes = [0; mem::size_of::<MessageHeader>()];
        match stream.read(&mut in_header_bytes) {
            Err(e) => {
                eprintln!("Failed to read message header from client: {}", e);
                break;
            },
            Ok(num_bytes) => {
                if num_bytes != in_header_bytes.len() {
                    eprintln!(
                        "Expected {} message header bytes; got {}",
                        in_header_bytes.len(),
                        num_bytes,
                    );
                    break;
                }
            }
        };

        // Parse raw bytes into a message header.
        let in_header = MessageHeader::from_bytes(&in_header_bytes).unwrap();

        // Read in raw bytes from the Unix stream (for the message payload).
        let mut in_payload_bytes: Vec<u8> = vec![0; in_header.payload_length];
        match stream.read(&mut in_payload_bytes) {
            Err(e) => {
                eprintln!("Failed to read message payload from client: {}", e);
            },
            Ok(num_bytes) => {
                if num_bytes != in_payload_bytes.len() {
                    eprintln!(
                        "Expected {} message payload bytes; got {}",
                        in_payload_bytes.len(),
                        num_bytes,
                    );
                    break;
                }
            },
        }
        let in_payload = String::from_utf8(in_payload_bytes).unwrap();
        println!("{}", in_payload);
        let out_payload = in_payload;

        // Build outgoing message header.
        let out_header = MessageHeader {
            status: MessageStatus::OkDone,
            payload_length: out_payload.len()
        };

        // Send outgoing message header.
        stream.write(out_header.as_bytes()).unwrap();
        stream.flush().unwrap();

        // Send outgoing message header.
        stream.write(out_payload.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    println!("Closing client connection");
}
