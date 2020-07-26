use std::io::{Read, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::{mem, io};

extern crate bongodb;
use bongodb::message::{MessageHeader, MessageStatus};

const UNIX_SOCKET_PATH: &str = "/tmp/bongodb.sock";
const CLIENT_PROMPT: &str = "BongoDB> ";

fn main() {
    // Setup stdin.
    let stdin_reader = BufReader::new(io::stdin());

    // Setup Unix socket stream.
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).unwrap();

    // Add interactive marker for tty input.
    // TODO: Check if stdin is tty.
    print!("{}", CLIENT_PROMPT);
    io::stdout().flush().unwrap();

    for line in stdin_reader.lines() {
        match line {
            Err(err) => {
                eprintln!("Failed to read stdin: {}", err);
                break;
            },
            Ok(line) => {
                if line.len() < 1 {
                    // Ignore empty lines of input.
                    continue;
                }

                // Build outgoing message header.
                let out_header = MessageHeader {
                    status: MessageStatus::OkDone,
                    payload_length: line.len(),
                };

                // Send outgoing message header.
                stream.write(out_header.as_bytes()).unwrap();
                stream.flush().unwrap();

                // Send outgoing message payload.
                stream.write(line.as_bytes()).unwrap();
                stream.flush().unwrap();

                // Read in raw bytes from the Unix stream (for the message header).
                let mut in_header_bytes = [0; mem::size_of::<MessageHeader>()];
                match stream.read(&mut in_header_bytes) {
                    Err(e) => {
                        eprintln!("Failed to read message header from server: {}", e);
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
                
                // Prepare for more user input.
                print!("{}", CLIENT_PROMPT);
                io::stdout().flush().unwrap();
            },
        }
    }
}
