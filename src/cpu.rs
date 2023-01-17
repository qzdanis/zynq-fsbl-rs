use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr;

extern {
    static __bss_start__: *mut u32;
    static __bss_end__: *mut u32;
}

global_asm!(include_str!("vectors.s"), options(raw));

// panic handler: loops processor
#[panic_handler]
fn panic(_p: &PanicInfo<'_>) -> ! {
    unsafe { asm!("wfe", options(noreturn)); }
}

// reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    unsafe {
        asm!(
            "ldr sp, =__proc0_svc_stack",
            "msr CPSR, #0x1f",
            "ldr sp, =__proc0_usr_stack",
        );
    }

    let mut bss = __bss_start__;
    while bss != __bss_end__ {
        ptr::write(bss, 0);
        bss = bss.add(1);
    }

    super::main();

    asm!("wfe", options(noreturn));
}
