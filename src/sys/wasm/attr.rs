use std::io;

use super::{Termios, BorrowedFd};

pub fn get_terminal_attr(_fd: BorrowedFd) -> io::Result<Termios> {
    todo!()
}

pub fn set_terminal_attr(_fd: BorrowedFd, _termios: &Termios) -> io::Result<()> {
    todo!()
}

pub fn raw_terminal_attr(_termios: &mut Termios) {
    todo!()
}
