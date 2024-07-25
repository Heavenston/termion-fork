use std::io;

use js_sys::wasm_bindgen;
use wasm_bindgen::JsValue;

pub mod attr;
pub mod size;
pub mod tty;

/// Wrapper around a JsValue
pub struct Termios(pub JsValue);

impl Termios {
    fn call0(&self, name: &str) -> JsValue {
        let val: js_sys::Function =
            js_sys::Reflect::get(&self.0, &name.into())
            .unwrap().into();
        val.call0(&self.0).unwrap()
    }

    /// Calls the raw_terminal_attr method of the js value
    pub fn raw_terminal_attr(&self) {
        self.call0("raw_terminal_attr");
    }
}

impl Clone for Termios {
    fn clone(&self) -> Self {
        Self(self.call0("clone"))
    }
}

impl From<wasm_bindgen::JsValue> for Termios {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self(value)
    }
}

impl From<Termios> for wasm_bindgen::JsValue {
    fn from(val: Termios) -> Self {
        val.0
    }
}

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
