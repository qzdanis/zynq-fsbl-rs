/*
 * boot.s
 *
 * System initialization code
 */
    .syntax unified
    .arm

/* CPU modes
    .equ CPU_MODE_USER, 0x10
    .equ CPU_MODE_FIQ,  0x11
    .equ CPU_MODE_IRQ,  0x12
    .equ CPU_MODE_SVC,  0x13
    .equ CPU_MODE_ABT,  0x17
    .equ CPU_MODE_UND,  0x1b
    .equ CPU_MODE_SYS,  0x1f
*/

    .section .vectors, "ax", %progbits
    b reset_handler
    b undef_handler
    b svc_handler
    b prefetch_abort_handler
    b data_abort_handler
    b hypervisor_handler
    b irq_handler
    b fiq_handler

    .text
reset_handler:
    bl Reset

shutdown:
undef_handler:
svc_handler:
prefetch_abort_handler:
data_abort_handler:
hypervisor_handler:
irq_handler:
fiq_handler:
    wfe

    .end
