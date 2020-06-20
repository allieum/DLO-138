use core::fmt::{Display, Write, LowerHex};

const _RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;
const BLACK: u16 = 0x0000;

const WAVE_COLOR: u16 = PASTEL_PINK;
const BG_COLOR: u16 = BLACK;

pub unsafe fn draw_waves() {
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
    const Y_OFFSET: i16 = - crate::adafruit_lcd::HEIGHT / 2;

    // ugh
    crate::adafruit_lcd::HEIGHT + Y_OFFSET - (voltage as i16 - ZERO_VOLTAGE) / ADC_SCALE
}

// needed?
pub unsafe fn _blink_message<T: Display + LowerHex>(msg: T) {
    static mut S: crate::debug::FixedCStr = crate::debug::FixedCStr::new();

    // Clear last message
    _draw_text(&S, BG_COLOR); // todo only do this if changed

    S._clear();

    write!(&mut S, "{:x}", msg).expect("didn't work");

    _draw_text(&S, WAVE_COLOR);
}

// todo accept &str and do conversion somewhere, maybe from/into
//unsafe fn draw_message(msg: &str) {
pub unsafe fn draw_message(msg: &crate::debug::FixedCStr) {
    let lcd = crate::adafruit_lcd::get();

    lcd.fill_screen(PASTEL_BLUE);

    lcd.set_cursor(75, 30);
    lcd.set_text_color(BLACK);
    lcd.set_text_size(1);

    lcd.print(msg.as_ptr());
}


pub unsafe fn _draw_text(msg: &crate::debug::FixedCStr, color: u16) {
    let lcd = crate::adafruit_lcd::get();

    lcd.set_cursor(110, 30);
    lcd.set_text_color(color);
    lcd.set_text_size(2);

    lcd.print(msg.as_ptr());
}
