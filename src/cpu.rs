use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr;

// panic handler: loops processor
#[panic_handler]
fn panic(_p: &PanicInfo<'_>) -> ! {
    loop {}
}

// bss start and end addresses
extern {
    static __bss_start__: *mut u32;
    static __bss_end__: *mut u32;
}

// include vector table
global_asm!(include_str!("vectors.s"), options(raw));

// CPU modes
// CPU_MODE_USER    0x10
// CPU_MODE_FIQ     0x11
// CPU_MODE_IRQ     0x12
// CPU_MODE_SVC     0x13
// CPU_MODE_ABT     0x17
// CPU_MODE_UND     0x1b
// CPU_MODE_SYS     0x1f

// reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // put CPU into SYS mode
    asm!(
        "ldr sp, =__proc0_svc_stack",
        "msr CPSR, #0x1f",
        "ldr sp, =__proc0_usr_stack",
    );

    // initialize bss
    let mut bss = __bss_start__;
    while bss != __bss_end__ {
        ptr::write(bss, 0);
        bss = bss.add(1);
    }

    // execute main
    super::main();

    loop {}
}

// undefined instruction handler
#[no_mangle]
pub unsafe extern "C" fn Undef() -> ! {
    loop {}
}

// supervisor call handler
#[no_mangle]
pub unsafe extern "C" fn SVC() -> ! {
    loop {}
}

// prefetch abort handler
#[no_mangle]
pub unsafe extern "C" fn Prefetch_abort() -> ! {
    loop {}
}

// data abort handler
#[no_mangle]
pub unsafe extern "C" fn Data_abort() -> ! {
    loop {}
}

// IRQ interrupt handler
#[no_mangle]
pub unsafe extern "C" fn IRQ() -> ! {
    loop {}
}

// FIQ interrupt handler
#[no_mangle]
pub unsafe extern "C" fn FIQ() -> ! {
    loop {}
}
