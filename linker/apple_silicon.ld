/* SSL 3.2 Linker Script - Apple Silicon (M1/M2/M3/M4)
 * For bare-metal on Apple Silicon Macs
 * 
 * IMPORTANT: Apple Silicon has unique memory architecture
 * - 16KB pages (not 4KB like x86-64)
 * - Kernel typically loaded at 0x800000000 (32GB offset)
 * - Requires m1n1 or similar bootloader chain
 */

ENTRY(_start)

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)

/* Apple Silicon memory map starts high */
MEMORY
{
    /* iBoot loads kernel here by default */
    DRAM (rwx) : ORIGIN = 0x800000000, LENGTH = 4G
}

SECTIONS
{
    /* Apple Silicon requires 16KB page alignment */
    
    /* Boot code */
    .text.boot ALIGN(16K) : {
        __text_boot_start = .;
        *(.text.boot)
        *(.text.init)
        __text_boot_end = .;
    } > DRAM
    
    /* Exception vectors (page aligned) */
    .text.vectors ALIGN(16K) : {
        __vectors_start = .;
        *(.text.vectors)
        __vectors_end = .;
    } > DRAM
    
    /* Main code */
    .text ALIGN(16K) : {
        __text_start = .;
        *(.text)
        *(.text.*)
        __text_end = .;
    } > DRAM
    
    /* Read-only data */
    .rodata ALIGN(16K) : {
        __rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        __rodata_end = .;
    } > DRAM
    
    /* Initialized data */
    .data ALIGN(16K) : {
        __data_start = .;
        *(.data)
        *(.data.*)
        __data_end = .;
    } > DRAM
    
    /* BSS */
    .bss ALIGN(16K) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    } > DRAM
    
    /* Stack (128 KB, 16-byte aligned) */
    . = ALIGN(16);
    __stack_bottom = .;
    . += 128K;
    __stack_top = .;
    
    /* Heap */
    . = ALIGN(16K);
    __heap_start = .;
    
    __kernel_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

/* Apple Silicon specific asserts */
ASSERT(__text_boot_start >= 0x800000000, "Kernel not in Apple Silicon memory region")
ASSERT(__bss_end - __bss_start < 64M, "BSS too large for initial mapping")
