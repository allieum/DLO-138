#![no_std]

#[macro_use]
pub mod debug;
pub mod ctypes;
pub mod draw;
pub mod lcd;
pub mod pins;
// todo maybe rename to board or something
pub mod stm32_peripherals;
pub mod sample;

// todo arduino feature flag
//pub mod adafruit_lcd;
//pub mod bindings;
