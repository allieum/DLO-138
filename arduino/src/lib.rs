#![no_std]
#![feature(never_type)]
#![feature(const_mut_refs)]

use dro138::{
    serial,
    bindings,
    ctypes::{c_char, c_void},
    debug,
    draw,
    adafruit_lcd,
    lcd,
    pins,
    sample,
    stm32_peripherals
};

// todo make a single extern fn (ie have rust take control up front)
//      maybe wouldn't need singletons?
//      also look into cortex_m::singleton macro
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd_ptr: *mut c_void, print_serial: fn(*const c_char)) {
    debug::init(print_serial);
    adafruit_lcd::init(lcd_ptr);
    stm32_peripherals::init();

    lcd::init();

    serial!("rusty serial {}", 4.20);
}

#[no_mangle]
pub unsafe extern "C" fn draw_waves() {
    draw::draw_waves();
}

#[no_mangle]
pub unsafe extern "C" fn sample_wave() {
    sample::sample_wave();
}
