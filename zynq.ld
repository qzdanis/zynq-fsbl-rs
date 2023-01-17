OUTPUT_FORMAT ("elf32-littlearm")

MEMORY {
    SRAM_0 : ORIGIN = 0x00000000, LENGTH = 0x00030000 /* 192k */
    SRAM_1 : ORIGIN = 0xffff0000, LENGTH = 0x00010000 /*  64k */
}

__stack_size = 0x1000;

__heap_end = __stext + LENGTH(SRAM_0);
__heap_start = end;

SECTIONS {
    .text : {
        __stext = .;
        KEEP(*(.vectors))
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.glue_7)
        *(.glue_7t)
        __etext = .;
    } >SRAM_0

    .data : {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*)
        __edata = .;
    } >SRAM_0

    .bss (NOLOAD) : {
        . = ALIGN(4);
        __bss_start__ = .;
        *(.bss .bss.*)
        __bss_end__ = .;
    } >SRAM_0

    . = ALIGN(4);
    end = .;

    .stack (NOLOAD) : {
        . += __stack_size;
        __proc0_svc_stack = .;
        . += __stack_size;
        __proc0_usr_stack = .;
    } >SRAM_1 =0x00000000
}
