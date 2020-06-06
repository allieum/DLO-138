#![no_std]

// todo, all needed? could put them in build.rs prepend too
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(alloc_error_handler)]

extern crate alloc;

//use alloc::str::String;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::fmt::Write;
use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

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
unsafe fn candy_panic(info: &PanicInfo) -> ! {
    //let args = info.message().unwrap();
    //let msg = format!("{:?}", args);
    let location = info.location().unwrap();
    let (file, line, column) = (location.file(), location.line(), location.column());

    let mut s = HackStr::new();

    write!(&mut s, "panic @ {} {}:{}", file, line, column).expect("didn't work");

    // let msg = format!("{} {}:{}", file, line, column);
    //    draw_message("pls panic friends\0");
    draw_message(&s);
    // draw_message(msg);

    loop {};

//     let lcd = adafruit_lcd();

//     let msg = info.payload().downcast_ref::<&str>().unwrap();

//     lcd.fill_screen(PASTEL_BLUE);

//     lcd.set_cursor(110, 30);
//     lcd.set_text_color(BLACK);
//     lcd.set_text_size(2);

// //    lcd.print(msg);

//     lcd.print("hello scope");

//    panic!("figure out what this is for in my life !!!");
}

// having these as Option may be silly
// todo screaming snake case for statics & consts
static mut PERIPHERALS: Option<stm32f103::Peripherals> = None;
static mut ADAFRUIT_LCD: Option<Adafruit_TFTLCD_8bit_STM32> = None;
static mut PRINT: Option<fn(*const c_char)> = None;

unsafe fn peripherals() -> &'static stm32f103::Peripherals {
    PERIPHERALS.as_ref().unwrap()
}

unsafe fn adafruit_lcd() -> &'static mut Adafruit_TFTLCD_8bit_STM32 {
    ADAFRUIT_LCD.as_mut().unwrap()
}

const _RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;
const BLACK: u16 = 0x0000;

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

struct HackStr {
    length: usize,
    pub buf: [c_char; 100]
}

impl HackStr {
    pub fn new() -> HackStr {
	HackStr {
	    length: 0,
	    buf: [0; 100]
	}
    }

    pub fn as_cstr(&self) -> *const c_char {
	self.buf.as_ptr()
    }
}


impl Write for HackStr {
    // todo bound check
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	for c in s.chars() {
	    self.buf[self.length] = c as c_char;
	    self.length += 1;
	}
	Ok(())
    }
}

struct HackVec {
    pub vec: Vec<c_char>
}

impl HackVec {
    pub fn new() -> HackVec {
	HackVec {
	    vec: Vec::new()
	}
    }

    pub fn as_cstr(&self) -> *const c_char {
	self.vec.as_ptr()
    }
}

impl Write for HackVec {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	for c in s.chars() {
	    self.vec.push(c as c_char);
	}
	self.vec.push(0);
	Ok(())
    }
}



// todo remove print
#[no_mangle]
pub unsafe extern "C" fn init_rust(lcd: *mut c_void, print: fn(*const c_char)) {
    let lcd = *(lcd as *mut Adafruit_TFTLCD_8bit_STM32);
    ADAFRUIT_LCD = Some(lcd);
    PERIPHERALS = Some(stm32f103::Peripherals::take().unwrap());
    PRINT = Some(print);

    //draw_message("strings are things\0");

    // Initialize the allocator BEFORE you use it
    //    let start = cortex_m_rt::heap_start() as usize;
    // 20kb = 20480 - 1024 = 19456
    adafruit_lcd().fill_screen(BG_COLOR);
    // let start = 5192; //19_456; // probably this doesn't work without marking segment as heap @ compile time... could look up for arduino
    // let size = 1024; // in bytes
    // ALLOCATOR.init(start, size);

//    adafruit_lcd().fill_screen(PASTEL_BLUE);
}

const SAMPLE_DEPTH: usize = 2048;
static mut WAVE_SAMPLES: [u16; SAMPLE_DEPTH] = [0; SAMPLE_DEPTH];

// todo sample mod
#[no_mangle]
pub unsafe extern "C" fn sample_wave() {
//    panic!("gosh...");
    for sample in WAVE_SAMPLES.iter_mut() {
	*sample = sample_adc1();
    }
}

pub unsafe fn sample_adc1() -> u16 {
    // Wait for a conversion to complete
    while peripherals().ADC1.sr.read().eoc().is_not_complete() {}

    // Get data register bits
    peripherals().ADC1.dr.read().data().bits()
}

const WAVE_COLOR: u16 = PASTEL_PINK;
const BG_COLOR: u16 = BLACK;

// todo draw mod
// todo rename draw_wave
#[no_mangle]
pub unsafe extern "C" fn draw_waves(_draw_cwaves: extern "C" fn()) {
    // todo system for storing drawn lines?

    // Pick a window centered in the middle of the samples array
    const START_SAMPLE: usize = (SAMPLE_DEPTH - SCREEN_WIDTH) / 2;
    static mut OLD_POINTS: [i16; SCREEN_WIDTH + 1] = [0; SCREEN_WIDTH + 1]; // +1 ?

    OLD_POINTS[..].windows(2).enumerate().for_each(|(i, pair)| {
	// todo could probably share code w below
	let x = (i + 1) as i16;
	if let [y1, y2] = pair {
	    adafruit_lcd().draw_line(x, *y1, x, *y2, BG_COLOR);
	}
    });

    // Draw vertical line segments connecting adjacent samples
    // todo this normalizes each voltage twice.. maybe preprocess then use zip with askew slices?
    WAVE_SAMPLES[START_SAMPLE .. START_SAMPLE + SCREEN_WIDTH]
	.windows(2)
	.map(normalize_voltage_pair)
        .enumerate()
        .for_each(|(i, (y1, y2))| {
	    let x = (i + 1) as i16;
	    // todo try drawFast fn
	    adafruit_lcd().draw_line(x, y1, x, y2, WAVE_COLOR);
	    OLD_POINTS[i] = y1;
	});
}

// todo accept &str and do conversion somewhere, maybe from/into
//unsafe fn draw_message(msg: &str) {
unsafe fn draw_message(msg: &HackStr) {
    let lcd = adafruit_lcd();

    lcd.fill_screen(PASTEL_BLUE);

    lcd.set_cursor(110, 30);
    lcd.set_text_color(BLACK);
    lcd.set_text_size(2);

    lcd.printc(msg.as_cstr());

//    lcd.print("hello scope");

}

fn normalize_voltage_pair(pair: &[u16]) -> (i16, i16) {
    match pair {
	&[v1, v2] => (normalize_voltage(v1), normalize_voltage(v2)),
	_ => panic!("gosh..")
    }
}

fn normalize_voltage(voltage: u16) -> i16 {
    const ADC_SCALE: i16 = 4; // ?
    const ZERO_VOLTAGE: i16 = 1985; // ?
    const Y_OFFSET: i16 = -30;

    // ugh
    SCREEN_HEIGHT + Y_OFFSET - (voltage as i16 - ZERO_VOLTAGE) / ADC_SCALE
}

// todo put this and bindings as adafruit mod
const SCREEN_WIDTH: usize = 320; // could be member fns
const SCREEN_HEIGHT: i16 = 240;

impl Adafruit_TFTLCD_8bit_STM32 {
    pub unsafe fn print(&mut self, msg: &str) {
	self._base._base.print2(msg.as_ptr() as *const i8);
    }

    // meh
    pub unsafe fn printc(&mut self, msg: *const i8) {
	self._base._base.print2(msg);
    }

    pub unsafe fn set_cursor(&mut self, x: i16, y: i16) {
	self._base.cursor_x = x;
	self._base.cursor_y = y;
    }

    pub unsafe fn set_text_color(&mut self, color: u16) {
	self._base.textcolor = color;
	self._base.textbgcolor = color;
    }

    pub unsafe fn set_text_size(&mut self, s: u8) {
	self._base.setTextSize(s);
    }

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
