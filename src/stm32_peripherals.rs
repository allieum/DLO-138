use stm32f1::stm32f103::GPIOA;

use stm32f1xx_hal::{pac::Peripherals, prelude::*, serial::{Config, Serial}};

use core::fmt::Write;

// static mut SINGLETON: Option<stm32f103::Peripherals> = None;


// fn set_high() {
//     // NOTE(unsafe) atomic write to a stateless register
//     Ok(unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << self.i)) })
// }

// fn set_low() {
//     // NOTE(unsafe) atomic write to a stateless register
//     Ok(unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) })
// }



#[no_mangle]
pub unsafe extern "C" fn blinka() {
//    let peripherals = Peripherals::take().unwrap();

    (*GPIOA::ptr()).bsrr.write(|w| w.bs15().clear_bit());
//    peripherals.GPIOA.bsrr.write(|w| w.bs15().clear_bit());
    serial!("blink?");
    (*GPIOA::ptr()).bsrr.write(|w| w.bs15().set_bit());
    serial!("blank..");
}

pub fn init() {
    let peripherals = Peripherals::take().unwrap();

    unsafe { setup_serial(peripherals) };

    // todo need macro_rules to pass peripherals around multiple places to break this up?
    //_setup_adc_dma(&mut peripherals);

//    SINGLETON = pac::Peripherals::take();
}

// pub unsafe fn get() -> &'static stm32f103::Peripherals {
//     SINGLETON.as_ref().unwrap()
// }

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
