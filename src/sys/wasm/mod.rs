use std::io;

pub mod attr;
pub mod size;
pub mod tty;

pub type Termios = wasm_bindgen::JsValue;
pub type BorrowedFd<'a, W> = &'a W;

/// Fake wasm implementation of std::os::fd::AsFd
pub trait AsFd {
    /// Should return the current termios config
    fn get_termios(&self) -> io::Result<Termios>;
    /// Should set the termios config to the given one
    fn set_termios(&self, new: &Termios) -> io::Result<()>;

    /// Borrow an fd
    fn as_fd(&self) -> BorrowedFd<'_, Self> {
        self
    }
}

impl<T: AsFd> AsFd for &T {
    fn get_termios(&self) -> io::Result<Termios> {
        T::get_termios(self)
    }

    fn set_termios(&self, new: &Termios) -> io::Result<()> {
        T::set_termios(self, new)
    }
}

impl<T: AsFd> AsFd for &mut T {
    fn get_termios(&self) -> io::Result<Termios> {
        T::get_termios(self)
    }

    fn set_termios(&self, new: &Termios) -> io::Result<()> {
        T::set_termios(self, new)
    }
}

pub trait AsRawFd {
    
}

impl<T> AsRawFd for T {
    
}
