/*
 * vectors.s
 *
 * Interrupt vector table for Cortex-A9
 */
    .syntax unified
    .arm

    .section .vectors, "ax", %progbits
    b Reset
    b Undef
    b SVC
    b Prefetch_abort
    b Data_abort
    b Reset
    b IRQ
    b FIQ

    .end
