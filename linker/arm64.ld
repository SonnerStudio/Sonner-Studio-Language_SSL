/* SSL 3.2 Linker Script - ARM64 / AArch64 Bare Metal
 * For Raspberry Pi 4, Pine64, and generic ARM64 platforms
 * 
 * Memory Layout varies by platform, default assumes 2MB start
 */

ENTRY(_start)

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)

SECTIONS
{
    /* Kernel starts at 2 MB (leaves space for firmware/DTB) */
    . = 2M;
    
    /* Boot code - must be first, initial execution */
    .text.boot ALIGN(4K) : {
        __text_boot_start = .;
        *(.text.boot)
        *(.text.init)
        __text_boot_end = .;
    }
    
    /* Exception vectors (must be 2KB aligned on ARM64) */
    .text.vectors ALIGN(2K) : {
        __vectors_start = .;
        *(.text.vectors)
        __vectors_end = .;
    }
    
    /* Main code section */
    .text ALIGN(4K) : {
        __text_start = .;
        *(.text)
        *(.text.*)
        __text_end = .;
    }
    
    /* Read-only data */
    .rodata ALIGN(4K) : {
        __rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        __rodata_end = .;
    }
    
    /* Initialized data */
    .data ALIGN(4K) : {
        __data_start = .;
        *(.data)
        *(.data.*)
        __data_end = .;
    }
    
    /* Uninitialized data (BSS) */
    .bss ALIGN(4K) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    }
    
    /* Stack (64 KB, 16-byte aligned for AAPCS64) */
    . = ALIGN(16);
    __stack_bottom = .;
    . += 64K;
    __stack_top = .;
    
    /* Heap start */
    . = ALIGN(4K);
    __heap_start = .;
    
    /* Kernel end marker */
    __kernel_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

ASSERT(__bss_end - __bss_start < 16M, "BSS section too large")
ASSERT(. < 512M, "Kernel too large")
