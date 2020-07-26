use std::{mem, slice};

/// Error codes
#[derive(Debug)]
pub enum MessageErr {
    InvalidBytes,
}

/// The status of a Unix socket message.
pub enum MessageStatus {
    OkDone,
}

/// A Unix socket message between the client and server.
pub struct MessageHeader {
    pub status: MessageStatus,
    pub payload_length: usize,
}

impl MessageHeader {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const MessageHeader as *const u8,
                mem::size_of::<MessageHeader>(),                
            )
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<&MessageHeader, MessageErr> {
        let (prefix, message_headers, suffix) = unsafe {
            bytes.align_to::<MessageHeader>()
        };

        if !prefix.is_empty() || !suffix.is_empty() || message_headers.len() != 1 {
            return Err(MessageErr::InvalidBytes);
        }

        Ok(&message_headers[0])
    }
}
