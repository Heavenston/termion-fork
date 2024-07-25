use std::io;

use wasm_bindgen::prelude::*;
use js_sys::Function;

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
    let func: Function = js_sys::Reflect::get(
        termios, &"raw_terminal_attr".into()
    ).unwrap().into();
    func.call0(&JsValue::NULL).unwrap();
}
