#![no_std]
#![feature(never_type)]
#![feature(const_mut_refs)]

mod bindings;
mod ctypes;
mod draw;
#[macro_use]
mod debug;
mod adafruit_lcd;
mod lcd;
mod pins;
mod stm32_peripherals;
mod sample;

use ctypes::{c_char, c_void};

// todo make a single extern fn (ie have rust take control up front)
//      maybe wouldn't need singletons?
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd_ptr: *mut c_void, print_serial: fn(*const c_char)) {
    debug::init(print_serial);
    adafruit_lcd::init(lcd_ptr);
    stm32_peripherals::init();

    lcd::init();

    // let lcd = adafruit_lcd::get();
    // lcd.fill_screen(draw::BG_COLOR);

    //    debug::print_serial("rusty serial");
    serial!("rusty serial {}", 2);
    serial!("hi");
}
