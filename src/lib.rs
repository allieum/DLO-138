#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub extern fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

const RUST_COLOR: u16 = 0xEB00;

#[no_mangle]
pub extern fn rust_colored_candy() -> u16 {
    RUST_COLOR
}

//#[cfg(test)]
//mod tests {
    //#[test]
    //fn it_works() {
    //    assert_eq!(2 + 2, 4);
    //}
//}
