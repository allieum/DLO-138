MAIN_INO=      DRO-138.ino
OUTPUT_DIR=    ./build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

# Maintain original contents of compiler.c.elf.extra_flags, overridden below
LD_DIR=        ${PWD}/hardware/STM32/STM32F1/variants/generic_stm32f103c/ld
RUST_LIB=      ${PWD}/target/thumbv7m-none-eabi/release/libdro138.a


# more varable.. also init submodule rule somewhere

all: compile deploy

# mess with specifying output
rust:
	cargo build --release --target=thumbv7m-none-eabi

# figure out params to exclude stdlib etc
dro138.h:
	cbindgen --config cbindgen.toml --crate dro138 --output dro138.h

# todo comment, var
compile: ${OUTPUT_DIR}
	arduino-builder -build-options-file build.options.json -verbose -prefs='custom.dro138.staticlib="${RUST_LIB}"' -build-path ${OUTPUT_DIR} ${MAIN_INO}

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy: compile
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf ${OUTPUT_DIR}/*
