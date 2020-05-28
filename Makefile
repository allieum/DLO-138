MAIN_INO=      DRO-138.ino
OUTPUT_DIR=    ./build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

RUST_H=        ./src/dro138.h
RUST_LIB=      ${PWD}/target/thumbv7m-none-eabi/release/libdro138.a

TFTLIB_H=      Adafruit_TFTLCD_8bit_STM32.h
TFTLIB_PATH=   ./src/TFTLib/${TFTLIB_H}
# rename to hpp to make bindgen happy
TFTLIB_TMP=    ${OUTPUT_DIR}/${TFTLIB_H}pp

# Copied from compile step output for use in bindgen
ARDUINO_BUILD_FLAGS= --sysroot=/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/arm-none-eabi -std=gnu++11 -nostdlib -mcpu=cortex-m3 -mthumb  -DDEBUG_LEVEL=DEBUG_NONE -DBOARD_generic_stm32f103c -DVECT_TAB_ADDR=0x8000000 -DERROR_LED_PORT=GPIOC -DERROR_LED_PIN=13 -DF_CPU=72000000L -DARDUINO=10812 -DARDUINO_GENERIC_STM32F103C -DARDUINO_ARCH_STM32F1 -DCONFIG_MAPLE_MINI_NO_DISABLE_DEBUG -DMCU_STM32F103CB -D__STM32F1__ -DMCU_STM32F103CB  -D__STM32F1__ -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/include -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/stm32f1/include -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/usb/stm32f1 -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/usb/usb_lib -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/cores/maple -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/variants/generic_stm32f103c -I/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/lib/gcc/arm-none-eabi/4.8.3/include -I/home/allie/Arduino/libraries/Adafruit_GFX_Library
#ARDUINO_BUILD_FLAGS= --sysroot=/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/arm-none-eabi -std=gnu++11 -nostdlib -mcpu=cortex-m3 -mthumb -march=armv7-m -DDEBUG_LEVEL=DEBUG_NONE -DBOARD_generic_stm32f103c -DVECT_TAB_ADDR=0x8000000 -DERROR_LED_PORT=GPIOC -DERROR_LED_PIN=13 -DF_CPU=72000000L -DARDUINO=10812 -DARDUINO_GENERIC_STM32F103C -DARDUINO_ARCH_STM32F1 -DCONFIG_MAPLE_MINI_NO_DISABLE_DEBUG -DMCU_STM32F103CB -D__STM32F1__ -DMCU_STM32F103CB  -D__STM32F1__ -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/include -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/stm32f1/include -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/usb/stm32f1 -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/system/libmaple/usb/usb_lib -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/cores/maple -I/home/allie/computers/scope/source/DRO-138/hardware/STM32/STM32F1/variants/generic_stm32f103c -I/home/allie/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/4.8.3-2014q1/lib/gcc/arm-none-eabi/4.8.3/include -I/home/allie/Arduino/libraries/Adafruit_GFX_Library

# more varable.. also init submodule rule somewhere

all: clean compile deploy

# mess with specifying output
rust:
	cargo build --release --target=thumbv7m-none-eabi

cbindgen: ${RUST_H}

# figure out params to exclude stdlib etc
${RUST_H}:
	cbindgen --config cbindgen.toml --crate dro138 --output ${RUST_H}

bindgen: copy-tftlib
	bindgen --use-core ${TFTLIB_TMP} -- ${ARDUINO_BUILD_FLAGS}

copy-tftlib:
	cp ${TFTLIB_PATH} ${TFTLIB_TMP}

# todo comment, var
compile: ${OUTPUT_DIR} rust cbindgen
	arduino-builder -build-options-file build.options.json -verbose -prefs='custom.dro138.staticlib="${RUST_LIB}"' -build-path ${OUTPUT_DIR} ${MAIN_INO}

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy:
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf ${OUTPUT_DIR}/* ${RUST_H}
