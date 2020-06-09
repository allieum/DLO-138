pub const SAMPLE_DEPTH: usize = 2048;
pub static mut WAVE_SAMPLES: [u16; SAMPLE_DEPTH] = [0; SAMPLE_DEPTH];

#[no_mangle]
pub unsafe extern "C" fn sample_wave() {
    for sample in WAVE_SAMPLES.iter_mut() {
    	*sample = sample_adc1();
    }
}

// todo replace with DMA
unsafe fn sample_adc1() -> u16 {
    let peripherals = crate::stm32_peripherals::get();

    // Wait for a conversion to complete
    while peripherals.ADC1.sr.read().eoc().is_not_complete() {}

    // Get data register bits
    peripherals.ADC1.dr.read().data().bits()
}
