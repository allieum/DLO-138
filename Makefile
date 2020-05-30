MAIN_INO=      DRO-138.ino
OUTPUT_DIR=    ./build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

# todo consider renaming to bindings.h? and put bindings.rs in variable
RUST_H=        ./src/dro138.h
RUST_LIB=      ${PWD}/target/thumbv7m-none-eabi/release/libdro138.a

# todo revisit
TFTLIB_H=      Adafruit_TFTLCD_8bit_STM32.h
TFTLIB_PATH=   ./src/TFTLib/${TFTLIB_H}
# rename to hpp to make bindgen happy
TFTLIB_TMP=    ${OUTPUT_DIR}/${TFTLIB_H}pp

# more varable.. also init submodule rule somewhere

all: clean compile deploy

# mess with specifying output
rust:
	cargo build --release # --target=thumbv7m-none-eabi

cbindgen:
	cbindgen --config cbindgen.toml --crate dro138 --output ${RUST_H}

#bindgen: copy-tftlib
#	bindgen --use-core ${TFTLIB_TMP} -- ${ARDUINO_BUILD_FLAGS}

copy-tftlib:
	cp ${TFTLIB_PATH} ${TFTLIB_TMP}

# todo comment, var
compile: ${OUTPUT_DIR} cbindgen rust
	arduino-builder -build-options-file build.options.json -verbose -prefs='custom.dro138.staticlib="${RUST_LIB}"' -build-path ${OUTPUT_DIR} ${MAIN_INO}

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy:
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf ${OUTPUT_DIR}/* ${RUST_H} src/bindings.rs target/thumbv7m-none-eabi
