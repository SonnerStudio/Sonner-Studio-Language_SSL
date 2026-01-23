/* SSL 3.2 Linker Script - x86-64 Bare Metal / ZetaCore
 * For use with GRUB2, Limine, or custom bootloader
 * 
 * Memory Layout:
 * 0x00000000 - 0x000FFFFF : Reserved (Real Mode, BIOS, etc.)
 * 0x00100000 - 0x???????? : Kernel (starts at 1 MB)
 */

ENTRY(_start)

/* Output format for different targets */
OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)

SECTIONS
{
    /* Kernel starts at 1 MB (standard for x86-64 kernels) */
    . = 1M;
    
    /* Multiboot2 header must be in first 32KB */
    .multiboot2 ALIGN(8) : {
        KEEP(*(.multiboot2))
    }
    
    /* Boot code - executed first, aligned to 4KB page */
    .text.boot ALIGN(4K) : {
        __text_boot_start = .;
        *(.text.boot)
        *(.text.init)
        __text_boot_end = .;
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
    
    /* Uninitialized data (BSS) - must be zeroed at boot */
    .bss ALIGN(4K) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    }
    
    /* Stack (64 KB, 16-byte aligned for x86-64 ABI) */
    . = ALIGN(16);
    __stack_bottom = .;
    . += 64K;
    __stack_top = .;
    
    /* Heap start marker */
    . = ALIGN(4K);
    __heap_start = .;
    
    /* Kernel end marker */
    __kernel_end = .;
    
    /* Discard unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

/* Assertions to catch common errors */
ASSERT(__bss_end - __bss_start < 16M, "BSS section too large (>16MB)")
ASSERT(__text_end - __text_start < 64M, "Text section too large (>64MB)")
ASSERT(. < 256M, "Kernel too large (>256MB)")
