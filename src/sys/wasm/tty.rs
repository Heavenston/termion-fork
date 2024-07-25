use super::AsRawFd;
use std::{fs, io};

/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(_stream: &T) -> bool {
    true
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    unimplemented!()
}
