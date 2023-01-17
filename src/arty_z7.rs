//! Digilent Arty-Z7 module
mod io;
pub mod prelude;

use core::ffi::{c_char, c_int, CStr};
use core::fmt;

#[link(name = "ps7")]
extern "C" {
    fn ps7_init() -> c_int;
    fn ps7_post_config() -> c_int;
    fn getPS7MessageInfo(key: u32) -> *mut c_char;
}

pub struct Board {}

impl Board {
    pub fn init() -> Board {
        let mut ret = unsafe { ps7_init() };
        io::uart0_init();

        let msg = unsafe { CStr::from_ptr(getPS7MessageInfo(ret as u32)) };
        io::uart0_writeln(msg.to_bytes());

        if ret != 0 {
            panic!();
        }

        ret = unsafe { ps7_post_config() };

        if ret != 0 {
            let msg = unsafe { CStr::from_ptr(getPS7MessageInfo(ret as u32)) };
            io::uart0_writeln(msg.to_bytes());

            panic!();
        }

        Board {}
    }
}

impl fmt::Write for Board {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        io::uart0_write(s.as_bytes());

        if s.ends_with("\n") {
            io::uart0_write(b"\r");
        }

        Ok(())
    }
}
