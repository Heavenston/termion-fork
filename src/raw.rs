//! Managing raw mode.
//!
//! Raw mode is a particular state a TTY can have. It signifies that:
//!
//! 1. No line buffering (the input is given byte-by-byte).
//! 2. The input is not written out, instead it has to be done manually by the programmer.
//! 3. The output is not canonicalized (for example, `\n` means "go one line down", not "line
//!    break").
//!
//! It is essential to design terminal programs.
//!
//! # Example
//!
//! ```rust,no_run
//! use termion::raw::IntoRawMode;
//! use std::io::{Write, stdout};
//!
//! let mut stdout = stdout().into_raw_mode()?;
//! write!(stdout, "Hey there.").unwrap();
//! # std::io::Result::Ok(())
//! ```

use std::{
    io::{self, Write},
    ops,
};

use sys::attr::{get_terminal_attr, raw_terminal_attr, set_terminal_attr};
use sys::{Termios, AsFd};

/// The timeout of an escape code control sequence, in milliseconds.
pub const CONTROL_SEQUENCE_TIMEOUT: u64 = 100;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
///
/// Restoring will entirely bring back the old TTY state.
pub struct RawTerminal<W: Write + AsFd> {
    prev_ios: Termios,
    output: Option<W>,
}

impl<W: Write + AsFd> RawTerminal<W> {
    /// Returns the owned output
    pub fn into_inner(mut self) -> W {
        let output = self.output.take().unwrap();

        let _ = set_terminal_attr(output.as_fd(), &self.prev_ios);

        output
    }
}

impl<W: Write + AsFd> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        let Some(output) = self.output.take()
        else { return; };
        let _ = set_terminal_attr(output.as_fd(), &self.prev_ios);
    }
}

impl<W: Write + AsFd> ops::Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        self.output.as_ref().unwrap()
    }
}

impl<W: Write + AsFd> ops::DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        self.output.as_mut().unwrap()
    }
}

impl<W: Write + AsFd> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.as_mut().unwrap().flush()
    }
}

#[cfg(unix)]
mod unix_impl {
    use super::*;
    use std::os::unix::io::{AsFd, BorrowedFd};

    impl<W: Write + AsFd> AsFd for RawTerminal<W> {
        fn as_fd(&self) -> BorrowedFd {
            self.output.as_ref().unwrap().as_fd()
        }
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
pub trait IntoRawMode: Write + AsFd + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write + AsFd> IntoRawMode for W {
    fn into_raw_mode(self) -> io::Result<RawTerminal<W>> {
        let mut ios = get_terminal_attr(self.as_fd())?;
        let prev_ios = ios.clone();

        raw_terminal_attr(&mut ios);

        set_terminal_attr(self.as_fd(), &ios)?;

        Ok(RawTerminal {
            prev_ios,
            output: Some(self),
        })
    }
}

impl<W: Write + AsFd> RawTerminal<W> {
    /// Temporarily switch to original mode
    pub fn suspend_raw_mode(&self) -> io::Result<()> {
        set_terminal_attr(self.as_fd(), &self.prev_ios)?;
        Ok(())
    }

    /// Temporarily switch to raw mode
    pub fn activate_raw_mode(&self) -> io::Result<()> {
        let mut ios = get_terminal_attr(self.as_fd())?;
        raw_terminal_attr(&mut ios);
        set_terminal_attr(self.as_fd(), &ios)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{stdout, Write};

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_into_raw_mode() {
        let mut out = stdout().into_raw_mode().unwrap();

        out.write_all(b"this is a test, muahhahahah\r\n").unwrap();

        drop(out);
    }
}
