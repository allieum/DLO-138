#![no_std]

// todo, all needed?
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod hack {
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub use core::ffi::c_void;
}

//include!("./bindings.rs");

mod bindings;

use bindings::Adafruit_GFX;

//use ::Adafruit_GFX;

use core::panic::PanicInfo;

#[panic_handler]
fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

const _RUST_COLOR: u16 = 0xEB00;
const PASTEL_PINK: u16 = 0xE4DD;
const PASTEL_BLUE: u16 = 0x9EDD;

#[no_mangle]
pub extern "C" fn sleepy(_gfx: Adafruit_GFX) {
}

#[no_mangle]
pub extern "C" fn pink_rust() -> u16 {
    PASTEL_PINK
}

#[no_mangle]
pub extern "C" fn blue_rust() -> u16 {
    PASTEL_BLUE
}

#[no_mangle]
pub extern "C" fn signal_chunk_factor() -> u16 {
    3
}

#[no_mangle]
pub extern "C" fn loadConfigFromRust(reset: bool, load_config: extern "C" fn(bool)) {
    load_config(reset);
}

#[no_mangle]
pub extern "C" fn draw_waves(draw_cwaves: extern "C" fn()) {
    draw_cwaves();
}
