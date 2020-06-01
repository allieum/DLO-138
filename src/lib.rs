#![no_std]

// todo, all needed?
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// todo this is a weird place for this... separate into its own file
// and import in build.rs to use as prepend? maybe
mod hack {
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub use core::ffi::c_void;
}

use hack::*;

mod bindings;

use bindings::Adafruit_TFTLCD_8bit_STM32;
use bindings::Adafruit_TFTLCD_8bit_STM32_fillScreen;

//use ::Adafruit_GFX;

use core::panic::PanicInfo;

#[panic_handler]
fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

const RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;

#[no_mangle]
pub extern "C" fn pink_rust() -> u16 {
    PASTEL_PINK
}

#[no_mangle]
pub extern "C" fn blue_rust() -> u16 {
    PASTEL_BLUE
}

#[no_mangle]
pub extern "C" fn signal_chunk_factor() -> u16 {
    3
}

#[no_mangle]
pub extern "C" fn loadConfigFromRust(reset: bool, load_config: extern "C" fn(bool)) {
    load_config(reset);
}

#[no_mangle]
pub unsafe extern "C" fn draw_waves(lcd: *mut c_void, draw_cwaves: extern "C" fn()) {
    Adafruit_TFTLCD_8bit_STM32_fillScreen(lcd, RUST_COLOR);
    let lcd = lcd as *mut Adafruit_TFTLCD_8bit_STM32;
    let lcd = &mut *lcd;
    draw_cwaves();
}

impl Adafruit_TFTLCD_8bit_STM32 {

}
