MAIN_INO=      DRO-138.ino
OUTPUT_DIR=    ./build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

# more varable

all: compile deploy

rust:
	cargo build --release --target=thumbv7m-none-eabi

header:
	cbindgen --config cbindgen.toml --crate dro138 --output dro138.h

compile:
	mkdir -p ${OUTPUT_DIR} && arduino-builder -build-options-file build.options.json -build-path ${OUTPUT_DIR} ${MAIN_INO}

deploy: compile
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf build/*
