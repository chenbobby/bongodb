use std::{fmt, io, mem, slice};

/// Error codes
pub enum Error {
    InvalidBytes,
    NotEnoughHeaderBytes(usize),
    NotEnoughPayloadBytes(usize),
    ReadFailure(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidBytes => write!(f, "Raw bytes are invalid for MessageHeader"),
            Error::NotEnoughHeaderBytes(n) => write!(f, "Failed to read sufficient bytes ({}) for MessageHeader", n),
            Error::NotEnoughPayloadBytes(n) => write!(f, "Failed to read sufficient bytes ({}) for message payload", n),
            Error::ReadFailure(e) => write!(f, "{}", e),
        }
    }
}

/// The status of a Unix socket message.
pub enum Status {
    OkDone,
}

/// A Unix socket message between the client and server.
pub struct MessageHeader {
    pub status: Status,
    pub payload_len: usize,
}

impl MessageHeader {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const MessageHeader as *const u8,
                mem::size_of::<MessageHeader>(),                
            )
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<&MessageHeader, Error> {
        let (prefix, message_headers, suffix) = unsafe {
            bytes.align_to::<MessageHeader>()
        };

        if !prefix.is_empty() || !suffix.is_empty() || message_headers.len() != 1 {
            return Err(Error::InvalidBytes);
        }

        Ok(&message_headers[0])
    }
}

/// Reads a full message (header and payload) and returns the payload as a `String`.
pub fn read<R>(reader: &mut R) -> Result<String, Error>
    where R: io::Read,
{
    // Read in raw bytes for the message header.
    let mut mh_bytes = [0; mem::size_of::<MessageHeader>()];
    match reader.read(&mut mh_bytes) {
        Err(e) => {
            return Err(Error::ReadFailure(e));
        },
        Ok(num_bytes) => {
            if num_bytes != mh_bytes.len() {
                return Err(Error::NotEnoughHeaderBytes(num_bytes));
            }
        }
    };

    // Parse raw bytes into a message header.
    let mh = MessageHeader::from_bytes(&mh_bytes)?;

    // Read in raw bytes for the message payload.
    let mut payload_bytes: Vec<u8> = vec![0; mh.payload_len];
    match reader.read(&mut payload_bytes) {
        Err(e) => {
            return Err(Error::ReadFailure(e));
        },
        Ok(num_bytes) => {
            if num_bytes != payload_bytes.len() {
                return Err(Error::NotEnoughPayloadBytes(num_bytes));
            }
        },
    }

    Ok(String::from_utf8(payload_bytes).unwrap())
}

/// Writes a message.
pub fn write<W>(writer: &mut W, status: Status, payload: &[u8])
    -> io::Result<()>
    where W: io::Write,
{
    // Build and write message header.
    let mh = MessageHeader {
        status,
        payload_len: payload.len(),
    };

    // Write the message header.
    writer.write(mh.as_bytes())?;
    writer.flush()?;

    // Write the message payload.
    writer.write(payload)?;
    writer.flush()?;

    Ok(())
}
