use core::fmt::{Display, Write, LowerHex};
//use core::fmt::Write;

const _RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;
pub const BLACK: u16 = 0x0000;

pub const WAVE_COLOR: u16 = PASTEL_PINK;
pub const BG_COLOR: u16 = BLACK;

// todo rename draw_wave?
#[no_mangle]
pub unsafe extern "C" fn draw_waves(_draw_cwaves: extern "C" fn()) {
    // todo system for storing drawn lines?

    // Pick a window centered in the middle of the samples array
    const START_SAMPLE: usize = (crate::sample::SAMPLE_DEPTH - crate::adafruit_lcd::WIDTH) / 2;
    static mut OLD_POINTS: [i16; crate::adafruit_lcd::WIDTH + 1] = [0; crate::adafruit_lcd::WIDTH + 1]; // +1 ?

    OLD_POINTS[..].windows(2).enumerate().for_each(|(i, pair)| {
	// todo could probably share code w below
	let x = (i + 1) as i16;
	if let [y1, y2] = pair {
	    crate::adafruit_lcd::get().draw_line(x, *y1, x, *y2, BG_COLOR);
	}
    });

    // Draw vertical line segments connecting adjacent samples
    // todo this normalizes each voltage twice.. maybe preprocess then use zip with askew slices?
    crate::sample::WAVE_SAMPLES[START_SAMPLE .. START_SAMPLE + crate::adafruit_lcd::WIDTH]
	.windows(2)
	.map(normalize_voltage_pair)
        .enumerate()
        .for_each(|(i, (y1, y2))| {
	    let x = (i + 1) as i16;
	    // todo try drawFast fn
	    crate::adafruit_lcd::get().draw_line(x, y1, x, y2, WAVE_COLOR);
	    OLD_POINTS[i] = y1;
	});

   // let cr = crate::stm32_peripherals::get().DMA1.ch1.cr.read().bits();

   //  crate::draw::blink_message(cr);
   //  crate::debug::print_serial(cr);
//    crate::draw::blink_message(crate::sample::WAVE_SAMPLES[0]);
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
    const Y_OFFSET: i16 = -crate::adafruit_lcd::HEIGHT / 2;

    // ugh
    crate::adafruit_lcd::HEIGHT + Y_OFFSET - (voltage as i16 - ZERO_VOLTAGE) / ADC_SCALE
}

pub unsafe fn blink_message<T: Display + LowerHex>(msg: T) {
    static mut S: crate::debug::HackStr = crate::debug::HackStr::new();

    // Clear last message
    draw_text(&S, BG_COLOR); // todo only do this if changed

    S.clear();

    write!(&mut S, "{:x}", msg).expect("didn't work");

    draw_text(&S, WAVE_COLOR);
}

// todo accept &str and do conversion somewhere, maybe from/into
//unsafe fn draw_message(msg: &str) {
pub unsafe fn draw_message(msg: &crate::debug::HackStr) {
    let lcd = crate::adafruit_lcd::get();

    lcd.fill_screen(PASTEL_BLUE);

    lcd.set_cursor(110, 30);
    lcd.set_text_color(BLACK);
    lcd.set_text_size(2);

    lcd.print(msg.as_cstr());
}


pub unsafe fn draw_text(msg: &crate::debug::HackStr, color: u16) {
    let lcd = crate::adafruit_lcd::get();

    lcd.set_cursor(110, 30);
    lcd.set_text_color(color);
    lcd.set_text_size(2);

    lcd.print(msg.as_cstr());
}
