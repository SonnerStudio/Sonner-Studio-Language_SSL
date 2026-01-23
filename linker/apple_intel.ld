/* SSL 3.2 Linker Script - Apple Intel (x86-64 macOS Hardware)
 * For bare-metal on Intel Macs (requires EFI bootloader)
 * 
 * Note: Apple uses Mach-O format, but for bare-metal we use ELF
 * The bootloader converts to appropriate format
 */

ENTRY(_start)

OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)

SECTIONS
{
    /* Apple firmware typically loads at 1MB like standard x86-64 */
    . = 1M;
    
    /* EFI handoff section */
    .efi_handoff ALIGN(8) : {
        *(.efi_handoff)
    }
    
    /* Boot code */
    .text.boot ALIGN(4K) : {
        __text_boot_start = .;
        *(.text.boot)
        *(.text.init)
        __text_boot_end = .;
    }
    
    /* Main code */
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
    
    /* BSS */
    .bss ALIGN(4K) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    }
    
    /* Stack (128 KB for Apple - larger for compatibility) */
    . = ALIGN(16);
    __stack_bottom = .;
    . += 128K;
    __stack_top = .;
    
    /* Heap */
    . = ALIGN(4K);
    __heap_start = .;
    
    __kernel_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
