use ili9341::{gpio::Gpio8Interface, Ili9341};
use embedded_hal::digital::v2::OutputPin;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{pac, prelude::*};
// use stm32f1::stm32f103::

//use crate::pins::*;
use crate::stm32_peripherals::DummyDelay;

use embedded_graphics::{
    drawable::Pixel,
    geometry::Point,
    pixelcolor::{
        raw::RawU16,
        Rgb565,
    },
    // primitives::Rectangle,
    // style::{PrimitiveStyle, Styled},
    DrawTarget,
};

use embedded_graphics::{
    egcircle, egtext, fonts::Font6x8, prelude::*, primitive_style,
    text_style,
};

use embedded_graphics::{
    egline,
    // pixelcolor::Rgb565,
    prelude::*,
    // primitive_style,
    primitives::Line,
    style::{PrimitiveStyle, Styled},
};


// Mappings taken from
// https://github.com/ardyesp/DLO-138/blob/master/src/TFTLib/Adafruit_TFTLCD_8bit_STM32.h
// fn tft_rdx() -> PB10 { PB10 } // Read
// fn tft_wrx() -> PC15 { PC15 } // Write
// fn tft_dcx() -> PC14 { PC14 } // Data/Command (sometimes Register Select (RS))
// fn tft_csx() -> PC13 { PC13 } // Chip Select
// fn tft_rst() -> PB11 { PB11 } // Reset

// fn tft_data_pins() -> &'static mut [&'static mut dyn OutputPin<Error = !>; 8] {
//     static mut PINS: [&'static mut dyn OutputPin<Error = !>; 8] = [
// 	&mut PB0,
// 	&mut PB1,
// 	&mut PB2,
// 	&mut PB3,
// 	&mut PB4,
// 	&mut PB5,
// 	&mut PB6,
// 	&mut PB7
//     ];

//     unsafe { &mut PINS }
// }


pub fn init() {
    let periphs = pac::Peripherals::take().unwrap();

    let mut rcc = periphs.RCC.constrain();
    let mut gpioa = periphs.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = periphs.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = periphs.GPIOC.split(&mut rcc.apb2);
    let mut afio = periphs.AFIO.constrain(&mut rcc.apb2);

    let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let mut led = pa15.into_push_pull_output(&mut gpioa.crh);
    led.set_low().unwrap();

    let mut tft_data_pins: [&mut dyn OutputPin<Error = _>; 8] = [
	&mut gpiob.pb0.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb1.into_push_pull_output(&mut gpiob.crl),
	&mut gpiob.pb2.into_push_pull_output(&mut gpiob.crl),
	&mut pb3.into_push_pull_output(&mut gpiob.crl),
	&mut pb4.into_push_pull_output(&mut gpiob.crl),
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
    let mut lcd = Ili9341::new(interface, tft_rst, &mut DummyDelay).unwrap();

    // for x in 0 .. lcd.width() {
    // 	for y in 0 .. lcd.height() {
    // 	    let px = Pixel(Point::new(x as i32, y as i32), Rgb565::from(RawU16::from(0)));
    // 	    px.draw(&mut lcd).unwrap();
    // 	}
    // }

    lcd.clear(Rgb565::BLACK).unwrap();
    // hprintln!("filled screen").unwrap();


    let c = egcircle!(
    center = (20, 20),
    radius = 100,
	style = primitive_style!(fill_color = Rgb565::RED)
    );
    let t = egtext!(
	text = "Hello Rust!",
	top_left = (20, 16),
	style = text_style!(font = Font6x8, text_color = Rgb565::GREEN)
    );

    c.draw(&mut lcd).unwrap();
    t.draw(&mut lcd).unwrap();

    let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(start = (10, 20), end = (30, 40));

    let stroke_line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
	start = (10, 20),
	end = (30, 40),
	style = primitive_style!(stroke_color = Rgb565::BLUE)
    );

    line.draw(&mut lcd).unwrap();
    stroke_line.draw(&mut lcd).unwrap();

    let mut color = 0;
    loop{
	lcd.clear(Rgb565::new(color, color * 2, color * 3)).unwrap();
	color += 50;
// 	let border = 100;
// 	for x in border .. lcd.width() - border {
// 	    for y in border .. lcd.height() - border {
// 		// let px = Pixel(Point::new(x as i32, y as i32), Rgb565::from(RawU16::from(color)));
// 		// px.draw(&mut lcd).unwrap();
// 		// color += 1;
// 		egline!(
// 		    start = (x as i32, y as i32),
// 		    end = (0, 0),
// 		    style = primitive_style!(stroke_color = Rgb565::BLUE)
// 		).draw(&mut lcd).unwrap();
// 	    }
// 	}
	// hprintln!("filled screen").unwrap();
    }
}


// use DisplayTarget
// fn fill_screen(&mut lcd: Ili9341<Gpio8Interface<&mut dyn OutputPin<Error = Infallible>, PC13<OutputPin<PushPull>>, PC15<OutputPin<PushPull>>, PC10<OutputPin<PushPull>>, PC14<OutputPin<PushPull>>>>, PB11<OutputPin<PushPull>>, color: u16) {
//     	for x in 0 .. lcd.width() {
// 	    for y in 0 .. lcd.height() {
// 		let px = Pixel(Point::new(x as i32, y as i32), Rgb565::from(RawU16::from(color)));
// 		lcd.draw_pixel(px).unwrap();
// 	    }
// 	}
// }
