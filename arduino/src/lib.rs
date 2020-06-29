#![no_std]

use dro138::{
    c_str,
    serial,
    ctypes::{c_char, c_void},
    debug,
    // draw,
// //    adafruit_lcd,
//     // lcd,
//     sample,
    stm32_peripherals,
};

// todo make a single extern fn (ie have rust take control up front)
//      maybe wouldn't need singletons?
//      also look into cortex_m::singleton macro
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd_ptr: *mut c_void, print_serial: fn(*const c_char)) {
    debug::init(print_serial);
    adafruit_lcd::init(lcd_ptr);
    adafruit_lcd::get().fill_screen(0);

    // draw::draw_message(&c_str!("nope, i am a scope"));
    serial!("but also a dope...");

    serial!("rusty serial {}", 4.21);

    stm32_peripherals::init();
}

// For tinydro.ino
#[no_mangle]
pub unsafe extern "C" fn tiny_init(print_serial: fn(*const c_char)) {
    debug::init(print_serial);

    serial!("made it this far");

//    stm32_peripherals::init();
}

// #[no_mangle]
// pub unsafe extern "C" fn draw_waves() {
//     draw::draw_waves();
// }

// #[no_mangle]
// pub unsafe extern "C" fn sample_wave() {
//     sample::sample_wave();
// }
