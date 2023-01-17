#![no_std]
#![no_main]
mod cpu;

#[cfg(feature = "arty-z7")]
use fsbl::arty_z7::prelude::*;

fn main() {
    let _board = Board::init();
}
