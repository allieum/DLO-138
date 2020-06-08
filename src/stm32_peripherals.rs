use stm32f1::stm32f103;

static mut SINGLETON: Option<stm32f103::Peripherals> = None;

pub unsafe fn init() {
    SINGLETON = stm32f103::Peripherals::take();
    setup_adc_dma();
}

pub unsafe fn get() -> &'static stm32f103::Peripherals {
    SINGLETON.as_ref().unwrap()
}

unsafe fn setup_adc_dma() {
    let peripherals = get();

    // Write the address of ADC1's data register to the
    // DMA's peripheral address register.
    let adc_address = &(peripherals.ADC1.dr) as *const _ as u32;
    peripherals.DMA1.ch1.par.write(|w| w.bits(adc_address));

    crate::error::print(adc_address);

    // Write buffer address to the memory address register
    let buffer_address = &crate::sample::WAVE_SAMPLES as *const _ as u32;
    peripherals.DMA1.ch1.mar.write(|w| w.bits(buffer_address));

    let buffer_size = crate::sample::SAMPLE_DEPTH as u32;
    peripherals.DMA1.ch1.ndtr.write(|w| w.bits(buffer_size));

    // Enable DMA requests for ADC1
    peripherals.ADC1.cr2.write(|w| w.dma().enabled());

    peripherals.DMA1.ch1.cr.write(|w| w
				  .pl().very_high()
				  .dir().from_peripheral()
				  .psize().bits16()
				  .msize().bits16()
				  .circ().enabled()
				  .minc().enabled()
				  .pinc().disabled()
				  // .tcie().enabled() // interrupt on complete?
				  .en().enabled());
}

// worth it? maybe do macro instead
// pub fn get_register_address(reg: &stm32f1::generic::Reg) {
