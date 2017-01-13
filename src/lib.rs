#![feature(const_fn, lang_items, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

// required lang item stubs
#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}


// main entry point
#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });

    loop {}
}

// fn print_something() {
//     let cc = ColorCode::new(Color::LightGreen, Color::Black);
//     let mut writer = Writer::new(cc);
//     writer.write_byte("Hello World");
// }
