use stm32f1::stm32f103;

// having these as Option may be silly
// todo screaming snake case for statics & consts
static mut PERIPHERALS: Option<stm32f103::Peripherals> = None;

unsafe fn peripherals() -> &'static stm32f103::Peripherals {
    PERIPHERALS.as_ref().unwrap()
}
