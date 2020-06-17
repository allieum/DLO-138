use ili9341::Ili9341;
use embedded_hal::digital::v2::OutputPin;

use crate::pins::*;

// Mappings taken from
// https://github.com/ardyesp/DLO-138/blob/master/src/TFTLib/Adafruit_TFTLCD_8bit_STM32.h
fn tft_rdx() -> PB10 { PB10 } // Read
fn tft_wrx() -> PC15 { PC15 } // Write
fn tft_dcx() -> PC14 { PC14 } // Data/Command (sometimes Register Select (RS))
fn tft_csx() -> PC13 { PC13 } // Chip Select
fn tft_rst() -> PB11 { PB11 } // Reset

fn tft_data_pins() -> &'static mut [&'static mut dyn OutputPin<Error = !>; 8] {
    static mut PINS: [&'static mut dyn OutputPin<Error = !>; 8] = [
	&mut PB0,
	&mut PB1,
	&mut PB2,
	&mut PB3,
	&mut PB4,
	&mut PB5,
	&mut PB6,
	&mut PB7
    ];

    unsafe { &mut PINS }
}

// delay could go in peripherals?
struct DummyDelay;
impl embedded_hal::blocking::delay::DelayMs<u16> for DummyDelay {
    fn delay_ms(&mut self, _ms: u16) {}
}

pub fn init() {
    let data_pins = tft_data_pins();
    let interface = ili9341::gpio::Gpio8Interface::new(data_pins, tft_csx(), tft_wrx(), tft_rdx(), tft_dcx());
    Ili9341::new(interface, tft_rst(), &mut DummyDelay).unwrap();
}
