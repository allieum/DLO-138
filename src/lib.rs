#![no_std]

mod bindings;
mod ctypes;
mod draw;
mod error;
mod adafruit_lcd;
mod stm32_peripherals;
mod sample;

// todo make a single extern fn (ie have rust take control up front)
//      maybe wouldn't need singletons?
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd_ptr: *mut ctypes::c_void) {
    adafruit_lcd::init(lcd_ptr);
    stm32_peripherals::init();

    let lcd = adafruit_lcd::get();
    lcd.fill_screen(draw::BG_COLOR);
}
