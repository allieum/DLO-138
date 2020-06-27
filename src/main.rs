#![no_std]
#![no_main]

// use core::fmt::Write;
use cortex_m_semihosting::{dbg, hprintln};
use cortex_m_rt::entry;
// use stm32f1xx_hal::{pac, prelude::*, serial::{Config, Serial}};

use dro138::{lcd, stm32_peripherals};


#[entry]
fn main() -> ! {
    // eh could still be macro_rules if laze..
    // todo ftdi still way faster... could try to use it first, fall
    // back if it fails / isn't set up yet. maybe implement fast dbg!
    hprintln!("hello, it's me, scope").unwrap();

    stm32_peripherals::init();

    loop {}
}
