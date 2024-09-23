ENTRY(_kernel_start)

SECTIONS
{
    . = 0xffffffff80000000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : {
        *(COMMON)
        *(.bss .bss.*)

        /* Align initial kernel heap to page boundary */
        . = ALIGN(4K);
        PROVIDE(_initial_kernel_heap_start = .);
        /* PROVIDE(_initial_kernel_heap_size = 1024 * 1024); */
        PROVIDE(_initial_kernel_heap_size = 1024 * 4096 * 100);
        . += _initial_kernel_heap_size;
    } :data
    . = ALIGN(8);
    . = . + 0x4000;
    LD_STACK_PTR = .;
}
