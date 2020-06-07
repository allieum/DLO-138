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


const WAVE_COLOR: u16 = PASTEL_PINK;
const BG_COLOR: u16 = BLACK;

// todo draw mod
// todo rename draw_wave
#[no_mangle]
pub unsafe extern "C" fn draw_waves(_draw_cwaves: extern "C" fn()) {
    // todo system for storing drawn lines?

    // Pick a window centered in the middle of the samples array
    const START_SAMPLE: usize = (crate::sample::SAMPLE_DEPTH - crate::lcd::SCREEN_WIDTH) / 2;
    static mut OLD_POINTS: [i16; crate::lcd::SCREEN_WIDTH + 1] = [0; crate::lcd::SCREEN_WIDTH + 1]; // +1 ?

    // circular mode etc. move adc1 setup here
    //peripherals().DMA1.ch1.par.write(|w| w.pa().bits(32));

    OLD_POINTS[..].windows(2).enumerate().for_each(|(i, pair)| {
	// todo could probably share code w below
	let x = (i + 1) as i16;
	if let [y1, y2] = pair {
	    //crate::lcd::draw_line(x, *y1, x, *y2, BG_COLOR);
	}
    });

    // Draw vertical line segments connecting adjacent samples
    // todo this normalizes each voltage twice.. maybe preprocess then use zip with askew slices?
    crate::sample::WAVE_SAMPLES[START_SAMPLE .. START_SAMPLE + crate::lcd::SCREEN_WIDTH]
	.windows(2)
	.map(normalize_voltage_pair)
        .enumerate()
        .for_each(|(i, (y1, y2))| {
	    let x = (i + 1) as i16;
	    // todo try drawFast fn
	    //adafruit_lcd().draw_line(x, y1, x, y2, WAVE_COLOR);
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
    const Y_OFFSET: i16 = -30;

    // ugh
    crate::lcd::SCREEN_HEIGHT + Y_OFFSET - (voltage as i16 - ZERO_VOLTAGE) / ADC_SCALE
}

// todo accept &str and do conversion somewhere, maybe from/into
//unsafe fn draw_message(msg: &str) {
pub unsafe fn draw_message(msg: &crate::error::HackStr) {
    // let lcd = adafruit_lcd();

    // lcd.fill_screen(PASTEL_BLUE);

    // lcd.set_cursor(110, 30);
    // lcd.set_text_color(BLACK);
    // lcd.set_text_size(2);

    // lcd.printc(msg.as_cstr());
}
