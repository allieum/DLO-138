// use stm32f1::stm32f103::GPIOA;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac::Peripherals, prelude::*, serial::{Config, Serial}};

use core::fmt::Write;

// static mut SINGLETON: Option<stm32f103::Peripherals> = Nonve;


// fn set_high() {
//     // NOTE(unsafe) atomic write to a stateless register
//     Ok(unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << self.i)) })
// }

// fn set_low() {
//     // NOTE(unsafe) atomic write to a stateless register
//     Ok(unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) })
// }

#[no_mangle]
pub unsafe extern "C" fn blinka(on: fn(), off: fn()) {

    hal_blink();

    // let p = Peripherals::steal();

    // // reset to low (on)
    // p.GPIOA.bsrr.write(|w| w.br15().set_bit());
    // let odr_bits = p.GPIOA.odr.read().bits() as u16;
    // serial!("{:#018b}", odr_bits);

    // // set to high (off)
    // p.GPIOA.bsrr.write(|w| w.bs15().set_bit());
    // let odr_bits = p.GPIOA.odr.read().bits() as u16;
    // serial!("{:#018b}", odr_bits);
}

pub fn init() {
    let peripherals = Peripherals::take().unwrap();

    unsafe { setup_serial(peripherals) };
}

// pub unsafe fn get() -> &'static stm32f103::Peripherals {
//     SINGLETON.as_ref().unwrap()
// }

unsafe fn hal_blink() {
    let dp = Peripherals::steal();
    let mut rcc = dp.RCC.constrain();

    serial!("got things");

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    serial!("got other things");

    let (pa15, _pb3, _pbq4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let mut led = pa15.into_push_pull_output(&mut gpioa.crh);

    serial!("got light things");

    led.set_low().unwrap();

    serial!("set light thing");

    led.set_high().unwrap();

    serial!("set light thing other way");
}

unsafe fn setup_serial(peripherals: Peripherals) {
    //serial!("hi");

    let mut flash = peripherals.FLASH.constrain();
    let mut rcc = peripherals.RCC.constrain();

    //serial!("i'm");

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    //serial!("slime");

    let mut afio = peripherals.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.apb2);

    //serial!("grime");

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    //serial!("possibly");

    let serial = Serial::usart1(
	peripherals.USART1,
	(tx, rx),
	&mut afio.mapr,
	Config::default().baudrate(115200.bps()),
	clocks,
	&mut rcc.apb2
    );

    //serial!("alive");

    let (mut tx, _rx) = serial.split();

    writeln!(tx, "meep").unwrap();
}

// todo rewrite with hal
unsafe fn _setup_adc_dma(peripherals: &mut Peripherals) {

    // Write the address of ADC1's data register to the
    // DMA's peripheral address register.
    let adc_address = &(peripherals.ADC1.dr) as *const _ as u32;
    peripherals.DMA1.ch1.par.write(|w| w.bits(adc_address));

    // Write buffer address to the memory address register
    let buffer_address = &crate::sample::WAVE_SAMPLES as *const _ as u32;
    peripherals.DMA1.ch1.mar.write(|w| w.bits(buffer_address));

    let buffer_size = 1;//crate::sample::SAMPLE_DEPTH as u32;
    peripherals.DMA1.ch1.ndtr.write(|w| w.bits(buffer_size));

    // Enable DMA requests for ADC1
    peripherals.ADC1.cr2.modify(|_, w| w.dma().enabled());

    peripherals.DMA1.ch1.cr.write(|w| w
				  .pl().very_high()
				  .dir().from_peripheral()
				  .psize().bits16()
				  .msize().bits16()
				  .circ().enabled()
				  .minc().enabled()
				  .pinc().disabled()
				  // .tcie().enabled() // interrupt on complete?
				  .en().enabled());

    // Start ADC
    // peripherals.ADC1.cr2.modify(|_, w| w.adon().enabled());
}

pub struct DummyDelay;
impl embedded_hal::blocking::delay::DelayMs<u16> for DummyDelay {
    fn delay_ms(&mut self, _ms: u16) {}
}
