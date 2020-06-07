#![no_std]

mod bindings;
mod ctypes;
mod draw;
mod error;
mod lcd;
mod peripherals;
mod sample;

// todo accept void ptr in lcd method

// todo make a single extern fn (ie have rust take control up front)
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd: *mut ctypes::c_void) {
    let lcd = *(lcd as *mut bindings::Adafruit_TFTLCD_8bit_STM32);
  //  ADAFRUIT_LCD = Some(lcd);
//    PERIPHERALS = Some(stm32f103::Peripherals::take().unwrap());
// todo do this elsewhere
//    adafruit_lcd().fill_screen(BG_COLOR);
}
