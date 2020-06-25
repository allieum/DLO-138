#![no_std]
#![no_main]

// use core::fmt::Write;
use cortex_m_rt::entry;
// use stm32f1::stm32f103::USART1;
// use stm32f1xx_hal::{pac, prelude::*, serial::{Config, Serial}};

//use dro138::stm32_peripherals;

// use nb::block;

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
// use stm32f1::stm32f103;


#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    // let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    // let dp = stm32f103::Peripherals::take().unwrap();
    // todo look into arduino gpio setup... clocks etc. maybe try other examples from hal crate too
    // dp.GPIOA.crh.write(|w| w.cnf15().alt_push_pull().mode15().output());
    // dp.GPIOA.bsrr.write(|w| w.br15().set_bit());

    // loop {}

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
//     let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

//     // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
//     // `clocks`
//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     // Acquire the GPIOC peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let (pa15, _pb3, _pbq4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

// //    unsafe { dp.AFIO.mapr.write(|w| w.swj_cfg().bits(0b100))};
// 	// afio.mapr2 // todo look into compare mapr2
//     // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
//     // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = pa15.into_push_pull_output(&mut gpioa.crh);

    led.set_low().unwrap();

    loop {}
//     // Configure the syst timer to trigger an update every second
//     let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

//     // Wait for the timer to trigger an update and change the state of the LED
//     loop {
//         block!(timer.wait()).unwrap();
//         led.set_high().unwrap();
//         block!(timer.wait()).unwrap();
//         led.set_low().unwrap();
//     }
}


// fn main() -> ! {
//     stm32_peripherals::init();

//     loop {}
// }
