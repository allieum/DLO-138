use ili9341::Ili9341;
use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{prelude::*};

use crate::pins::*;
use crate::stm32_peripherals::DummyDelay;

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


pub fn init() {
    let periphs = stm32f1xx_hal::pac::Peripherals::take().unwrap();
    let mut rcc = periphs.RCC.constrain();

    let mut gpioa = periphs.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = periphs.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = periphs.GPIOC.split(&mut rcc.apb2);


//    let get_output = |data_pin: &mut dyn OutputPin<Error = _> | data_pin.into_push_pull_output(&mut gpioa.crl);

    // todo: look into how to set these pups as output pins
    let tft_data_pins: [&mut dyn OutputPin<Error = _>; 8] = [
	&mut gpiob.pb0.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb1.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb2.into_push_pull_output(&mut gpiob.crl),
	// &mut gpiob.pb3.into_push_pull_output(&mut gpiob.crl),
	// &mut gpiob.pb4.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb7.into_push_pull_output(&mut gpiob.crl),
    ];

    let tft_rdx = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let tft_wrx = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);
    let tft_dcx = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);
    let tft_csx = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let tft_rst = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);

    let interface = ili9341::gpio::Gpio8Interface::new(&mut tft_data_pins, tft_csx, tft_wrx, tft_rdx, tft_dcx);
    Ili9341::new(interface, tft_rst, &mut DummyDelay).unwrap();
}
