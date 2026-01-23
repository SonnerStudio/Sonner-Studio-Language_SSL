/* SSL 3.2 Linker Script - Steam Deck (AMD APU)
 * Valve Steam Deck: AMD Zen 2 CPU + RDNA 2 GPU (Van Gogh APU)
 * 
 * Architecture: x86-64 with AMD-specific features
 * Notable: 
 * - 16GB unified RAM
 * - Custom AMD firmware
 * - UEFI boot
 */

ENTRY(_start)

OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)

SECTIONS
{
    /* Standard x86-64 kernel start */
    . = 1M;
    
    /* Multiboot2 for GRUB compatibility */
    .multiboot2 ALIGN(8) : {
        KEEP(*(.multiboot2))
    }
    
    /* Boot code */
    .text.boot ALIGN(4K) : {
        __text_boot_start = .;
        *(.text.boot)
        *(.text.init)
        __text_boot_end = .;
    }
    
    /* AMD-specific initialization */
    .text.amd_init ALIGN(4K) : {
        *(.text.amd_init)
        *(.text.gpu_init)
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
    
    /* GPU firmware/shaders (RDNA 2) */
    .gpu_data ALIGN(4K) : {
        __gpu_data_start = .;
        *(.gpu_data)
        *(.rdna2_shaders)
        __gpu_data_end = .;
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
    
    /* Stack (256 KB - Steam Deck has plenty of RAM for gaming workloads) */
    . = ALIGN(16);
    __stack_bottom = .;
    . += 256K;
    __stack_top = .;
    
    /* Heap */
    . = ALIGN(4K);
    __heap_start = .;
    
    /* Reserve space for GPU VRAM mapping */
    . = ALIGN(2M);  /* 2MB huge page aligned for GPU */
    __gpu_vram_start = .;
    . += 512M;      /* 512MB reserved for GPU VRAM */
    __gpu_vram_end = .;
    
    __kernel_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

/* Steam Deck specific asserts */
ASSERT(__stack_top - __stack_bottom >= 256K, "Stack too small for Steam Deck")
ASSERT(__gpu_vram_end <= 1G, "GPU VRAM mapping exceeds low memory")
