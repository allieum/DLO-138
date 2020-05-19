#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub extern fn candy_panic(_: &PanicInfo) -> ! {
    panic!("no more candy !!!");
}

#[no_mangle]
pub extern fn candy() -> u16 {
    48_886
}

//#[cfg(test)]
//mod tests {
    //#[test]
    //fn it_works() {
    //    assert_eq!(2 + 2, 4);
    //}
//}
