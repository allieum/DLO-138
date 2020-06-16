# todo etags

MAIN_INO=      DRO-138.ino
MAIN_INO_PATH= ./arduino/${MAIN_INO}
OUTPUT_DIR=    ${PWD}/build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

# todo consider renaming to bindings.h? and put bindings.rs in variable
RUST_H=        ./arduino/src/dro138.h
RUST_LIB=      ${PWD}/target/thumbv7m-none-eabi/release/libdro138.a
# RUST_VARS=    'RUSTFLAGS="-Z macro-backtrace"'

# more varable.. also init submodule rule somewhere

nobindings: compile deploy listen-serial

# todo make it so we don't have to clean here....
all: clean cbindgen compile deploy listen-serial

rust:
	cargo build --release

cbindgen:
	cbindgen --config cbindgen.toml --crate dro138 --output ${RUST_H}

# todo comment, var
compile: ${OUTPUT_DIR} rust
	arduino-builder -build-options-file arduino/build.options.json -verbose -prefs='custom.dro138.staticlib="${RUST_LIB}"' -build-path ${OUTPUT_DIR} ${MAIN_INO_PATH};

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy:
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

listen-serial:
	socat stdio /dev/ttyUSB0

clean:
	rm -rf ${OUTPUT_DIR}/* ${RUST_H} src/bindings.rs target/thumbv7m-none-eabi
