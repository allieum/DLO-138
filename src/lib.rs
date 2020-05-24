#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

const RUST_COLOR: u16 = 0xEB00;

#[no_mangle]
pub extern "C" fn rust_colored_candy(age: u16) -> u16 {
    RUST_COLOR + age
}

#[no_mangle]
pub extern "C" fn loadConfigFromRust(reset: bool, load_config: extern "C" fn(bool)) {
    load_config(reset);
}

//#[cfg(test)]
//mod tests {
    //#[test]
    //fn it_works() {
    //    assert_eq!(2 + 2, 4);
    //}
//}
