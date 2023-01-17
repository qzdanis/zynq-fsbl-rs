//! I/O routines.
use core::ptr;

const UART0_CR: *mut u32 = 0xe000_0000 as *mut u32;
const UART0_MR: *mut u32 = 0xe000_0004 as *mut u32;
const UART0_SR: *mut u32 = 0xe000_002c as *mut u32;
const UART0_BRG: *mut u32 = 0xe000_0018 as *mut u32;
const UART0_BRD: *mut u32 = 0xe000_0034 as *mut u32;
const UART0_FIFO: *mut u32 = 0xe000_0030 as *mut u32;

pub fn uart0_init() {
    unsafe {
        ptr::write_volatile(
            UART0_CR,
            (ptr::read_volatile(UART0_CR) & 0xffff_ffc3) | 0x0000_0028,
        );
        ptr::write_volatile(UART0_BRG, 0x0000_007c);
        ptr::write_volatile(UART0_BRD, 0x0000_0006);
        ptr::write_volatile(UART0_CR, 0x0000_0003);
        ptr::write_volatile(
            UART0_CR,
            (ptr::read_volatile(UART0_CR) & 0xffff_ffc3) | 0x0000_0014,
        );
        ptr::write_volatile(
            UART0_MR,
            (ptr::read_volatile(UART0_MR) & 0xffff_ff21) | 0x0000_0020,
        );
    }
}

pub fn uart0_write(s: &[u8]) {
    for byte in s {
        unsafe {
            loop {
                if (ptr::read_volatile(UART0_SR) & 0x0000_0010) == 0 {
                    break;
                }
            }

            ptr::write_volatile(UART0_FIFO, *byte as u32);
        }
    }
}

pub fn uart0_writeln(s: &[u8]) {
    uart0_write(s);
    uart0_write(b"\r\n");
}
