use core::{convert::Infallible, fmt::{Debug, Display, Write, LowerHex}};

const _RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;
const BLACK: u16 = 0x0000;

const WAVE_COLOR: u16 = PASTEL_PINK;
const BG_COLOR: u16 = BLACK;

use wyhash::wyrng;

use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::*,
    primitives::Line,
    style::{PrimitiveStyle, Styled}
};

use stm32f1xx_hal::{delay::Delay, prelude::*};

fn rgb(value: u16) -> Rgb565 {
    RawU16::new(value).into()
}

fn vertical_line(x: i32, y1: i32, y2: i32, color: Rgb565) -> Styled<Line, PrimitiveStyle<Rgb565>> {
    const EXTRA_CHUNK: i32 = 1;

    let (y1, y2) = if y1 > y2 {
	(y2, y1)
    } else {
	(y1, y2)
    };

    Line::new(Point::new(x, y1 - EXTRA_CHUNK), Point::new(x, y2 + EXTRA_CHUNK))
	.into_styled(PrimitiveStyle::with_stroke(color, 1))
}

pub fn draw_waves<E: Debug, D: DrawTarget<Rgb565, Error = E>>(samples: &[u16], lcd: &mut D, color: Rgb565, delay: &mut Delay) -> Rgb565 {
    // Pick a window centered in the middle of the samples array
    const WIDTH: usize = 320;
    const START_SAMPLE: usize = (512 - WIDTH) / 2;
    static mut OLD_POINTS: [u16; WIDTH + 1] = [0; WIDTH + 1]; // +1 ?

    let (mut r, mut g, mut b) = (color.r(), color.g(), color.b());
    let mut new_color = rgb(0);

    unsafe {
	let black = rgb(0);
	// OLD_POINTS[..].windows(2).enumerate().for_each(|(i, pair)| {
	//     // todo could probably share code w below
	//     let x = (i + 1) as i32;
	//     if let [y1, y2] = pair {
	// 	vertical_line(x, *y1 as i32, *y2 as i32, black).draw(lcd).unwrap();
	//     }
	// });


	static mut SEED: u64 = 42;
	static mut DELTA: i32 = 0;

	// Draw vertical line segments connecting adjacent samples
	// todo this normalizes each voltage twice.. maybe preprocess then use zip with askew slices?
	samples[START_SAMPLE .. START_SAMPLE + WIDTH]
	    .windows(2)
	    .map(normalize_voltage_pair)
            .enumerate()
            .for_each(|(i, (y1, y2))| {
		let x = (i + 1) as i32;
		// serial!("{} ({} {})", x, y1, y2);

		// Erase our tracks..
		// doesn't seem to quite work
		// .. probably a general draw accounting system is the way
		let (y1old, y2old) = (OLD_POINTS[i] as i32, OLD_POINTS[i + 1] as i32);
		vertical_line(x - 1, y1old, y2old, black).draw(lcd).unwrap();

		// todo make fn for this.. rng could be local static
		if wyrng(&mut SEED) & 0b1111 == 0 {
		    match DELTA + (y1 + y2) / 2 {
			0 => DELTA += 20,
			240 => DELTA -= -20,
			_ => DELTA += if wyrng(&mut SEED) & 0b1 == 0 { 1 } else { -1 }
		    };
		}
		let (y1, y2) = (y1 + DELTA as i32, y2 + DELTA as i32);

		static mut CDELTA: i32 = 1;
    		if wyrng(&mut SEED) & 0b1111 == 0 {
    		    r = (r as i32 + CDELTA) as u8;
    		    g = (g as i32 + CDELTA) as u8;
    		    b = (b as i32 + CDELTA) as u8;
    		}
    		new_color = Rgb565::new(r, g, b);

		vertical_line(x, y1 as i32, y2 as i32, new_color).draw(lcd).unwrap();
		OLD_POINTS[i] = y1 as u16;

		// todo codify run speed / framerate somewhere. hook various
		// params to keys
		delay.delay_ms(1 as u16);
	    });
    }

    new_color
}

fn normalize_voltage_pair(pair: &[u16]) -> (i32, i32) {
    match pair {
	&[v1, v2] => (normalize_voltage(v1), normalize_voltage(v2)),
	_ => panic!("gosh..")
    }
}

fn normalize_voltage(voltage: u16) -> i32 {
    const HEIGHT: u16 = 240;

    // to use half of screen, want to map range of adc onto 120 pixels
    const ZERO_VOLTAGE: u16 = 4095; // ?

    const Y_OFFSET: u16 = HEIGHT / 4;
    // const ADC_SCALE: u16 = ZERO_VOLTAGE / HEIGHT;
    const ADC_SCALE: u16 = 18;
    // ugh
    let y = (ZERO_VOLTAGE - voltage) / ADC_SCALE + Y_OFFSET;

    // serial!("({} - {}) / {} + {} = {}", ZERO_VOLTAGE, voltage, ADC_SCALE, Y_OFFSET, y);
    y as i32
}

// // needed?
// pub unsafe fn _blink_message<T: Display + LowerHex>(msg: T) {
//     static mut S: crate::debug::FixedCStr = crate::debug::FixedCStr::new();

//     // Clear last message
//     _draw_text(&S, BG_COLOR); // todo only do this if changed

//     S._clear();

//     write!(&mut S, "{:x}", msg).expect("didn't work");

//     _draw_text(&S, WAVE_COLOR);
// }

// // todo accept &str and do conversion somewhere, maybe from/into
// //unsafe fn draw_message(msg: &str) {
// pub unsafe fn draw_message(msg: &crate::debug::FixedCStr) {
//     let lcd = crate::adafruit_lcd::get();

//     lcd.fill_screen(PASTEL_BLUE);

//     lcd.set_cursor(75, 30);
//     lcd.set_text_color(BLACK);
//     lcd.set_text_size(1);

//     lcd.print(msg.as_ptr());
// }


// pub unsafe fn _draw_text(msg: &crate::debug::FixedCStr, color: u16) {
//     let lcd = crate::adafruit_lcd::get();

//     lcd.set_cursor(110, 30);
//     lcd.set_text_color(color);
//     lcd.set_text_size(2);

//     lcd.print(msg.as_ptr());
// }
