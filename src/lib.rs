#![no_std]

mod bindings;
mod ctypes;
mod draw;
mod debug;
mod adafruit_lcd;
mod stm32_peripherals;
mod sample;

use ctypes::{c_char, c_void};

// todo make a single extern fn (ie have rust take control up front)
//      maybe wouldn't need singletons?
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd_ptr: *mut c_void, print_serial: fn(*const c_char)) {
    adafruit_lcd::init(lcd_ptr);
    stm32_peripherals::init();
    debug::init(print_serial);

    let lcd = adafruit_lcd::get();
    lcd.fill_screen(draw::BG_COLOR);

    debug::print_serial("rusty serial");
}
