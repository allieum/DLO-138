#![no_std]
#![no_main]

// use core::fmt::Write;
use cortex_m_rt::entry;
// use stm32f1::stm32f103::USART1;
// use stm32f1xx_hal::{pac, prelude::*, serial::{Config, Serial}};

use dro138::stm32_peripherals;


#[entry]
fn main() -> ! {
    stm32_peripherals::init();

    loop {}
}
