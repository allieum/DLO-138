# todo etags, submodule init

ARDUINO_DIR=    ${PWD}/arduino
ARDUINO_OUT_DIR=${PWD}/build

ARDUINO_INO= 	${ARDUINO_DIR}/DRO-138.ino
ARDUINO_BIN=   	${ARDUINO_OUT_DIR}/DRO-138.ino.bin
ARDUINO_OPTIONS=${ARDUINO_DIR}/build.options.json

TINY_DIR=	${PWD}/tiny-arduino
TINY_INO=       ${TINY_DIR}/tinydro.ino
TINY_BIN=	${ARDUINO_OUT_DIR}/tinydro.ino.bin

SERIAL_DEVICE= 	/dev/ttyUSB0
BAUD_RATE=	115200

RUST_BUILD_DIR= target/thumbv7m-none-eabi/release

RUST_BIN_CRATE= dro138
RUST_BIN_OUT=   ${PWD}/${RUST_BUILD_DIR}
RUST_BIN=	${RUST_BIN_OUT}/${RUST_BIN_CRATE}

RUST_LIB_CRATE= arduino
RUST_LIB_OUT=   ${ARDUINO_DIR}/${RUST_BUILD_DIR}
RUST_LIB=      	${RUST_LIB_OUT}/librust.a
RUST_H=        	${ARDUINO_DIR}/src/rust.h
RUST_BINDINGS=  ${PWD}/src/bindings.rs

CBINDGEN_CONFIG=${PWD}/cbindgen.toml

ARDUINO_PREFS=  'custom.dro138.staticlib="${RUST_LIB}"'

nobindings: build-arduino deploy-arduino listen-serial

# todo make it so we don't have to clean here...
# (rustbindgen / cbindgen order dependency)
#
# - could potentially run cbindgen from build script
#   to get rid of command line dependency and enforce order / correctness ???
all: clean cbindgen build-arduino deploy-arduino listen-serial

tiny: cp-rust build-tiny-arduino deploy-tiny listen-serial

cp-rust: build-arduino-rust-lib
	cp ${RUST_H} ${TINY_DIR}

build-rust:
	cargo build --release

build-arduino-rust-lib:
	cd ${ARDUINO_DIR} && cargo build --release

build-arduino: ${ARDUINO_OUT_DIR} build-arduino-rust-lib
	arduino-builder -build-options-file ${ARDUINO_OPTIONS} -verbose -prefs=${ARDUINO_PREFS} -build-path ${ARDUINO_OUT_DIR} ${ARDUINO_INO}

build-tiny-arduino: ${ARDUINO_OUT_DIR} build-arduino-rust-lib
	arduino-builder -build-options-file ${ARDUINO_OPTIONS} -verbose -prefs=${ARDUINO_PREFS} -build-path ${ARDUINO_OUT_DIR} ${TINY_INO}

cbindgen:
	cd ${ARDUINO_DIR} && cbindgen --config ${CBINDGEN_CONFIG} --crate ${RUST_LIB_CRATE} --output ${RUST_H}

deploy-arduino:
	OUTPUT_BIN=${ARDUINO_BIN} make _deploy

deploy-tiny:
	OUTPUT_BIN=${TINY_BIN} make _deploy

deploy-rust:
	OUTPUT_BIN=${RUST_BIN} make _deploy

_deploy:
	stm32flash -b ${BAUD_RATE} -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

listen-serial:
	socat stdio ${SERIAL_DEVICE}

${ARDUINO_OUT_DIR}:
	mkdir -p ${ARDUINO_OUT_DIR}

clean:
	rm -rf ${ARDUINO_OUT_DIR}/* ${RUST_H} ${RUST_BINDINGS} target/thumbv7m-none-eabi
