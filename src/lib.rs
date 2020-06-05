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
    // todo this could display an error msg debug screen
    panic!("figure out what this is for in my life !!!");
}

// having these as Option may be silly
// todo screaming snake case for statics & consts
static mut PERIPHERALS: Option<stm32f103::Peripherals> = None;
static mut ADAFRUIT_LCD: Option<Adafruit_TFTLCD_8bit_STM32> = None;

unsafe fn peripherals() -> &'static stm32f103::Peripherals {
    PERIPHERALS.as_ref().unwrap()
}

unsafe fn adafruit_lcd() -> &'static Adafruit_TFTLCD_8bit_STM32 {
    ADAFRUIT_LCD.as_ref().unwrap()
}

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
pub unsafe extern "C" fn init_rust(lcd: *mut c_void) {
    let lcd = *(lcd as *mut Adafruit_TFTLCD_8bit_STM32);
    ADAFRUIT_LCD = Some(lcd);
    PERIPHERALS = Some(stm32f103::Peripherals::take().unwrap());
}

const sample_depth: usize = 2048;
static mut wave_samples: [u16; sample_depth] = [0; sample_depth];

// todo sample mod
#[no_mangle]
pub unsafe extern "C" fn sample_wave() {
    for sample in wave_samples.iter_mut() {
	*sample = sample_adc1();
    }
}

pub unsafe fn sample_adc1() -> u16 {
    // Wait for a conversion to complete
    while peripherals().ADC1.sr.read().eoc().is_not_complete() {}

    // Get data register bits
    peripherals().ADC1.dr.read().data().bits()
}

// todo draw mod
#[no_mangle]
pub unsafe extern "C" fn draw_waves(_draw_cwaves: extern "C" fn()) {

}

// todo put this and bindings as adafruit mod
const _screen_width: u16 = 320; // could be member fns
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
