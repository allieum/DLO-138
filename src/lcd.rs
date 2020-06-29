use crate::draw::draw_waves;
use core::fmt::Write;

use ili9341::{Ili9341, Orientation};
use embedded_hal::digital::v2::OutputPin;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{adc, delay::Delay, pac, prelude::*, serial::{Serial, Config}};
// use stm32f1::stm32f103::

use cortex_m::singleton;
use wyhash::wyrng;

use embedded_graphics::{
    // drawable::Pixel,
    // geometry::Point,
    pixelcolor::Rgb565,
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
    // prelude::*,
    // primitive_style,
    primitives::Line,
    style::{PrimitiveStyle, Styled},
};

pub fn init() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let clocks = rcc.cfgr
        .use_hse(8.mhz()) // high frequency external oscillator
	.sysclk(72.mhz())
	// .hclk(72.mhz()) // implied default
        .pclk1(36.mhz())
        // .pclk2(72.mhz()) // implied default
        // .adcclk(14.mhz()) // 14mhz max, 9mhz implied default
	.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    const SAMPLE_DEPTH: usize = 512;
    let dma_ch1 = dp.DMA1.split(&mut rcc.ahb).1;
    let adc1 = adc::Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);
    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    let tx_uart = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_uart = gpioa.pa10;

    let mut adc_samples = singleton!(: [u16; SAMPLE_DEPTH] = [0; SAMPLE_DEPTH]).unwrap();
    let mut adc_dma = adc1.with_dma(adc_ch0, dma_ch1);

    //serial!("possibly");

    let serial = Serial::usart1(
	dp.USART1,
	(tx_uart, rx_uart),
	&mut afio.mapr,
	Config::default().baudrate(115200.bps()),
	clocks,
	&mut rcc.apb2
    );

    let (tx_uart, _rx_uart) = serial.split();

    // serial!("this");
    // serial!("should");
    // serial!("be...");
    // serial!("sloww..");
    crate::debug::init(tx_uart);

    // writeln!(tx_uart, "meep").unwrap();
    serial!("meep..\nmeep\nmeeeeep");

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
    let mut lcd = Ili9341::new(interface, tft_rst, &mut delay).unwrap();
    lcd.set_orientation(Orientation::LandscapeFlipped).unwrap();


    // for x in 0 .. lcd.width() {
    // 	for y in 0 .. lcd.height() {
    // 	    let px = Pixel(Point::new(x as i32, y as i32), Rgb565::from(RawU16::from(0)));
    // 	    px.draw(&mut lcd).unwrap();
    // 	}
    // }

    lcd.clear(Rgb565::BLACK).unwrap();
    // hprintln!("filled screen").unwrap();


    // let c = egcircle!(
    // center = (20, 20),
    // radius = 100,
    // 	style = primitive_style!(fill_color = Rgb565::RED)
    // );
    // let t = egtext!(
    // 	text = "ooooOOOOoooOOOOoOOOOOOooooooo",
    // 	top_left = (20, 16),
    // 	style = text_style!(font = Font6x8, text_color = Rgb565::GREEN)
    // );

    // // c.draw(&mut lcd).unwrap();
    // t.draw(&mut lcd).unwrap();

    // let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(start = (10, 20), end = (30, 40));

    // let stroke_line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
    // 	start = (10, 20),
    // 	end = (30, 40),
    // 	style = primitive_style!(stroke_color = Rgb565::BLUE)
    // );


    // let w = lcd.width() as i32;

    // let mut seed = 42;
    // let mut delta: i32 = 1;

    // let mut r = wyrng(&mut seed) as u8;
    // let mut g = wyrng(&mut seed) as u8;
    // let mut b = wyrng(&mut seed) as u8;

    let mut color = Rgb565::new(0, 0, 0);
    loop {
	// let mut x = wyrng(&mut seed) as i32;
	// x = x % (w / 2) + w / 2;

	// slow (is it?),try not not do this here
	// todo troubleshoot whether adc inputs are registering
	let (filled_adc_samples, finished_adc_dma) = adc_dma.read(adc_samples).wait();
	adc_dma = finished_adc_dma;
	adc_samples = filled_adc_samples;

    	// if wyrng(&mut seed) & 0b1111 == 0 {
    	//     delta = -delta;

    	//     r = (r as i32 + delta) as u8;
    	//     g = (g as i32 - delta) as u8;
    	//     b = (b as i32 + delta) as u8;
    	// }


	// todo could disable print in release mode
	// serial!("{:?}", &adc_samples[0..20]);
	color = draw_waves(&adc_samples[..], &mut lcd, color, &mut delay);

	// delay.delay_ms(100 as u16);

	// 	for y in 0 .. lcd.height() {

    // 	    x = x + delta;

    // 	    delta = match x {
    // 		0 => 1,
    // 		320 => -1,
    // 		_ => delta
    // 	    };

    // 	    let width = 6;
    // 	    Line::new(Point::new(x - width / 2 as i32, y as i32),
    // 		      Point::new(x + width / 2 as i32, y as i32))
    // 		.into_styled(PrimitiveStyle::with_stroke(color, 1))
    // 	    	.draw(&mut lcd).unwrap();


    // 	}
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
