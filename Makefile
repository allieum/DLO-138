MAIN_INO=      ./arduino/DRO-138.ino
OUTPUT_DIR=    ${PWD}/build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

# todo consider renaming to bindings.h? and put bindings.rs in variable
RUST_H=        ./src/dro138.h
RUST_LIB=      ${PWD}/target/thumbv7m-none-eabi/release/libdro138.a

# more varable.. also init submodule rule somewhere

all: clean compile deploy

rust:
	cargo build --release

cbindgen:
	cbindgen --config cbindgen.toml --crate dro138 --output ${RUST_H}

# todo comment, var
compile: ${OUTPUT_DIR} cbindgen rust
	arduino-builder -build-options-file arduino/build.options.json -verbose -prefs='custom.dro138.staticlib="${RUST_LIB}"' -build-path ${OUTPUT_DIR} ${MAIN_INO};

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy:
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf ${OUTPUT_DIR}/* ${RUST_H} src/bindings.rs target/thumbv7m-none-eabi
