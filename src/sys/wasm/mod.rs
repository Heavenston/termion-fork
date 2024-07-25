use std::{io::Write, marker::PhantomData};

#[derive(Clone)]
pub struct Termios;

pub struct BorrowedFd<'a> {
    t: PhantomData<&'a ()>,
}

pub mod attr;
pub mod size;
pub mod tty;

/// Fake wasm implementation of std::os::fd::AsFd
pub trait AsFd {
    /// Borrow an fd
    fn as_fd(&self) -> BorrowedFd;
}

impl<T: Write> AsFd for T {
    fn as_fd(&self) -> BorrowedFd {
        BorrowedFd {
            t: Default::default(),
        }
    }
}

pub trait AsRawFd {
    
}

impl<T> AsRawFd for T {
    
}
