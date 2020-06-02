#![no_std]

// todo, all needed? could put them in build.rs prepend too
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use stm32f1::stm32f103;

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

use bindings::Adafruit_GFX_drawLine;
use bindings::Adafruit_TFTLCD_8bit_STM32;
use bindings::Adafruit_TFTLCD_8bit_STM32_fillScreen;

use core::panic::PanicInfo;

#[panic_handler]
fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

const peripherals: stm32f103::Peripherals = stm32f103::Peripherals::take().unwrap();

const _RUST_COLOR: u16 = 0xEB00;
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
//    Adafruit_TFTLCD_8bit_STM32_fillScreen(lcd, RUST_COLOR);
    let lcd = lcd as *mut Adafruit_TFTLCD_8bit_STM32;
    let mut lcd = *lcd;
    lcd.fill_screen(PASTEL_BLUE);
    draw_cwaves();

    let _sample_ready = peripherals.ADC1.sr.read().eoc().is_complete();

    // ADC data register
    // wow! todo figure out if it even gets data. & how to get 16 bits
    let _sample = peripherals.ADC1.dr.read().data().bits() as u16;
}

const _screen_width: u16 = 320;
const _screen_height: u16 = 240;

impl Adafruit_TFTLCD_8bit_STM32 {
    // todo look into typedefs and also from/into for pointer conversion
    pub unsafe fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u16) {
	let this = self as *mut Adafruit_TFTLCD_8bit_STM32 as *mut c_void;
	Adafruit_GFX_drawLine(this, x0, y0, x1, y1, color);
    }

    pub unsafe fn fill_screen(&mut self, color: u16) {
	let this = self as *mut Adafruit_TFTLCD_8bit_STM32 as *mut c_void;
	Adafruit_TFTLCD_8bit_STM32_fillScreen(this, color);
    }
}
