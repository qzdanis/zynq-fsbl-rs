#![no_std]
#![no_main]
mod cpu;

#[cfg(feature = "arty-z7")]
use fsbl::arty_z7::prelude::*;

use core::writeln;

fn main() {
    let mut board = Board::init();
    let _ = writeln!(board, "board initialization successful");
}
