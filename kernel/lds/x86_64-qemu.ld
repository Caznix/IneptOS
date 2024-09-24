/* Tell the linker that we want an x86_64 ELF64 output file */
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

ENTRY(_kernel_start)

/* Define the program headers we want so the bootloader gives us the right */
/* MMU permissions */
PHDRS
{
    null    PT_NULL    FLAGS(0) ;                   /* Null segment */
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
}

SECTIONS
{
    /* We wanna be placed in the topmost 2GiB of the address space, for optimisations */
    /* and because that is what the Limine spec mandates. */
    /* Any address in this region will do, but often 0xffffffff80000000 is chosen as */
    /* that is the beginning of the region. */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    /* Align .rodata to page boundary */
    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    /* Align .data to page boundary */
    . = ALIGN(4K);

    .data : {
        *(.data .data.*)
    } :data

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
}
