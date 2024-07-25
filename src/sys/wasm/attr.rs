use std::io;

use super::{AsFd, BorrowedFd, Termios};

pub fn get_terminal_attr<W: AsFd>(fd: BorrowedFd<'_, W>) -> io::Result<Termios> {
    fd.get_termios()
}

pub fn set_terminal_attr<W: AsFd>(
    fd: BorrowedFd<'_, W>,
    termios: &Termios
) -> io::Result<()> {
    fd.set_termios(termios)
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    termios.raw_terminal_attr();
}
