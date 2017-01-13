#![feature(const_fn, lang_items, unique)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

mod memory;
#[macro_use]
mod vga_buffer;

use memory::{AreaFrameAllocator, FrameAllocator};

// required lang item stubs
#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}


// main entry point
#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: {:#x}, size: {:#x}, flags: {:#b}",
                 section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    println!("kernel_start: {:#x}, kernel_end: {:#x}", kernel_start, kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("multiboot_start: {:#x}, multiboot_end: {:#x}", multiboot_start, multiboot_end);

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas()
        );

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }

    loop {}
}
