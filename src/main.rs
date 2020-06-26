#![no_std]
#![no_main]

// use core::fmt::Write;
use cortex_m_semihosting::{dbg, hprintln};
use cortex_m_rt::entry;
// use stm32f1::stm32f103::USART1;
// use stm32f1xx_hal::{pac, prelude::*, serial::{Config, Serial}};

//use dro138::stm32_peripherals;

use nb::block;

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
// use stm32f1::stm32f103;


#[entry]
fn main() -> ! {
    // eh could still be macro_rules if laze..
    // todo ftdi still way faster... could try to use it first, fall
    // back if it fails / isn't set up yet. maybe implement fast dbg!
    hprintln!("hello, it's me, scope").unwrap();

    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

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

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}


// fn main() -> ! {
//     stm32_peripherals::init();

//     loop {}
// }
