use std::{fs, io, thread};
use std::os::unix::net::{UnixStream, UnixListener};

use crate::UNIX_SOCKET_PATH;
use crate::message;

/// Runs the BongoDB server. The server will continually listen for Unix socket
/// connections until an interrupt or error occurs.
pub fn run() -> Result<(), io::Error> {
    // Delete Unix socket file if it already exists.
    match fs::remove_file(UNIX_SOCKET_PATH) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => (),    // File does not exist. Do nothing.
        Err(e) => return Err(e),    // Unknown error. Return from main.
        Ok(()) => (),    // File was successfully removed. Do nothing.
    }

    // Initialize and bind a Unix socket.
    let socket = UnixListener::bind(UNIX_SOCKET_PATH)?;

    // Begin listening for incoming connections.
    println!("\nBongoDB server is now listening on Unix socket {}", UNIX_SOCKET_PATH);
    for stream in socket.incoming() {
        match stream {
            Err(e) => return Err(e),
            Ok(mut stream) => {
                thread::spawn(move || handle_client(&mut stream));
            },
        }
    }

    Ok(())
}

/// Handles a new client connection on the Unix socket.
fn handle_client(stream: &mut UnixStream) {
    println!("\nNew client connected on Unix socket.");
    loop {
        // Read the incoming message.
        let in_payload = match message::read(stream) {
            Err(message::Error::NotEnoughHeaderBytes(0)) => break, // No more data.
            Err(e) => {
                eprintln!("Failed to read message: {}", e);
                break;
            },
            Ok(payload) => payload,
        };

        println!("{}", &in_payload);
        // TODO: Parse and process the payload as a DB query.
        let out_payload = in_payload;

        // Write the outgoing message.
        if let Err(e) = message::write(
            stream,
            message::Status::OkDone,
            out_payload.as_bytes(),
        ) {
            eprintln!("Failed to write message: {}", e);
            break;
        }
    }
    println!("Closing client connection on Unix socket.");
}
