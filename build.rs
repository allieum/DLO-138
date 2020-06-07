extern crate bindgen;

use std::path::PathBuf;

const HEADER_PATH: &str = "./arduino/src/TFTLib/Adafruit_TFTLCD_8bit_STM32.hpp";

const PREPEND_DIRECTIVES: &str = "#![allow(non_upper_case_globals)]\n\
				  #![allow(non_camel_case_types)]\n\
				  #![allow(non_snake_case)]\n\n";

// todo format into lines
const ARGS: &str = "--sysroot=/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/arm-none-eabi -std=gnu++11 -nostdlib -mcpu=cortex-m3 -mthumb -march=armv7-m -DDEBUG_LEVEL=DEBUG_NONE -DBOARD_generic_stm32f103c -DVECT_TAB_ADDR=0x8000000 -DERROR_LED_PORT=GPIOC -DERROR_LED_PIN=13 -DF_CPU=72000000L -DARDUINO=10812 -DARDUINO_GENERIC_STM32F103C -DARDUINO_ARCH_STM32F1 -DCONFIG_MAPLE_MINI_NO_DISABLE_DEBUG -DMCU_STM32F103CB -D__STM32F1__ -DMCU_STM32F103CB  -D__STM32F1__ -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/system/libmaple -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/system/libmaple/include -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/system/libmaple/stm32f1/include -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/system/libmaple/usb/stm32f1 -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/system/libmaple/usb/usb_lib -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/cores/maple -I/home/allie/computers/scope/source/DRO-138/arduino/hardware/STM32/STM32F1/variants/generic_stm32f103c -I/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/lib/gcc/arm-none-eabi/4.8.3/include -I/home/allie/Arduino/libraries/Adafruit_GFX_Library -I/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/arm-none-eabi/include";

fn main() {
    let bindings = bindgen::Builder::default()
        .header(HEADER_PATH)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
	.clang_args(ARGS.split(" "))
        .whitelist_type("Adafruit_TFTLCD_8bit_STM32")
	.use_core()
        .ctypes_prefix("crate::lcd::ctypes")
        .
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
