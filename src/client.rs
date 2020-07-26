use std::io::{self, BufRead, Write};
use std::os::unix::net::UnixStream;
use std::process;

use crate::UNIX_SOCKET_PATH;
use crate::message;

/// Runs the BongoDB client.
pub fn run() -> Result<(), io::Error> {
    // Setup Unix socket stream.
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH)?;

    // Add interactive marker for tty input.
    if unsafe { libc::isatty(libc::STDIN_FILENO) != 0 } { print_client_prompt()?; }

    // Read line-by-line from stdin.
    let mut stdin_reader = io::BufReader::new(io::stdin());
    loop {
        let mut payload = String::new();
        match stdin_reader.read_line(&mut payload) {
            Err(e) => return Err(e),
            Ok(0) => break, // Reader has reached EOF.
            Ok(_) => (),
        }

        // Pop off the newline character from the payload.
        payload.pop();

        if payload.len() > 0 {
            // Write the message.
            message::write(&mut stream, message::Status::OkDone, payload.as_bytes())?;

            // Read the response message.
            match message::read(&mut stream) {
                Err(e) => {
                    eprintln!("Failed to read message: {}", e);
                    process::exit(103);
                },
                Ok(payload) => println!("{}", &payload),
            }
        }

        // Add interactive marker for next line of tty input.
        if unsafe { libc::isatty(libc::STDIN_FILENO) != 0 } { print_client_prompt()?; }
    }

    Ok(())
}

/// Prints a prompt to the command line for user input.
fn print_client_prompt() -> io::Result<()> {
    print!("BongoDB> ");
    io::stdout().flush()?;
    Ok(())
}
