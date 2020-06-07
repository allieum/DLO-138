#![no_std]

use stm32f1::stm32f103;

mod bindings;
mod draw;
mod error;
mod lcd;
mod peripherals;
mod sample;

use core::panic::PanicInfo;

#[panic_handler]
unsafe fn candy_panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let (file, line, column) = (location.file(), location.line(), location.column());

    let mut s = HackStr::new();

    // todo abstract hackstr + write into another macro to learn how they work?
    write!(&mut s, "panic @ {} {}:{}", file, line, column).expect("didn't work");

    draw_message(&s);

    loop {};
}

// having these as Option may be silly
// todo screaming snake case for statics & consts
static mut PERIPHERALS: Option<stm32f103::Peripherals> = None;
static mut PRINT: Option<fn(*const c_char)> = None;

unsafe fn peripherals() -> &'static stm32f103::Peripherals {
    PERIPHERALS.as_ref().unwrap()
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

    // circular mode etc. move adc1 setup here
    //peripherals().DMA1.ch1.par.write(|w| w.pa().bits(32));

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
