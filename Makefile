# todo etags, submodule init

ARDUINO_DIR=	${PWD}/arduino
ARDUINO_MAIN= 	${ARDUINO_DIR}/DRO-138.ino

OUTPUT_DIR=    	${PWD}/build
OUTPUT_BIN=    	${OUTPUT_DIR}/DRO-138.ino.bin

SERIAL_DEVICE= 	/dev/ttyUSB0
BAUD_RATE=	115200

RUST_CRATE=     arduino
RUST_BINDINGS=  ${PWD}/src/bindings.rs
RUST_H=        	${ARDUINO_DIR}/src/rust.h
RUST_LIB=      	${ARDUINO_DIR}/target/thumbv7m-none-eabi/release/librust.a

CBINDGEN_CONFIG=${PWD}/cbindgen.toml

ARDUINO_OPTIONS=${PWD}/arduino/build.options.json
ARDUINO_PREFS=  'custom.dro138.staticlib="${RUST_LIB}"'

nobindings: compile deploy listen-serial

# todo make it so we don't have to clean here...
# (rustbindgen / cbindgen order dependency)
#
# - could potentially run cbindgen from build script
#   to get rid of command line dependency and enforce order / correctness ???
all: clean cbindgen compile deploy listen-serial

cbindgen:
	cd ${ARDUINO_DIR} && cbindgen --config ${CBINDGEN_CONFIG} --crate ${RUST_CRATE} --output ${RUST_H}

rust:
	cd ${ARDUINO_DIR} && cargo build --release

compile: ${OUTPUT_DIR} rust
	arduino-builder -build-options-file ${ARDUINO_OPTIONS} -verbose -prefs=${ARDUINO_PREFS} -build-path ${OUTPUT_DIR} ${ARDUINO_MAIN}

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

deploy:
	stm32flash -b ${BAUD_RATE} -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

listen-serial:
	socat stdio ${SERIAL_DEVICE}

clean:
	rm -rf ${OUTPUT_DIR}/* ${RUST_H} ${RUST_BINDINGS} target/thumbv7m-none-eabi
