// use stm32f1::stm32f103::GPIOA;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac::Peripherals, prelude::*, serial::{Config, Serial}, timer::Timer};

use core::fmt::Write;

use nb::block;

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
pub unsafe extern "C" fn blinka(_on: fn(), _off: fn()) {

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
    // let peripherals = Peripherals::take().unwrap();

    // unsafe { setup_serial(peripherals) };

    hal_blink();
}

// pub unsafe fn get() -> &'static stm32f103::Peripherals {
//     SINGLETON.as_ref().unwrap()
// }

fn hal_blink() {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let (pa15, _pb3, _pbq4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let mut led = pa15.into_push_pull_output(&mut gpioa.crh);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(4.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}

unsafe fn _setup_serial(peripherals: Peripherals) {
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
