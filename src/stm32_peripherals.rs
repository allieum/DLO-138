use stm32f1::stm32f103;

static mut SINGLETON: Option<stm32f103::Peripherals> = None;

pub unsafe fn init() {
    SINGLETON = stm32f103::Peripherals::take();
}

pub unsafe fn get() -> &'static stm32f103::Peripherals {
    SINGLETON.as_ref().unwrap()
}

// DMA config stuff, etc
