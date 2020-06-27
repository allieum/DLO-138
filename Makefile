# todo etags, submodule init, gdb?

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

# todo print individual segments. look into makefile function
GET_SIZE_ROW=	Total
GET_SIZE_CMD=	size -A --total ${RUST_BIN} | grep ${GET_SIZE_ROW} | grep -oE "[^ ]+$$" | numfmt --to=iec-i --suffix=B
SIZE_ARGS = 	-A ${RUST_BIN}

CBINDGEN_CONFIG=${PWD}/cbindgen.toml

ARDUINO_PREFS=  'custom.dro138.staticlib="${RUST_LIB}"'

FLASH_FLAGS=	 -b ${BAUD_RATE} -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

OPENOCD_FLAGS=   -f interface/stlink.cfg -f target/stm32f1x.cfg
OPENOCD_SCRIPT=  ./flash-and-run.expect
OPENOCD_KILL=    pkill openocd
TELNET_KILL=     pkill telnet

all: build-rust print-size swd

openocd:
	openocd ${OPENOCD_FLAGS}

swd: clean-prev-swd
#	# OpenOCD runs detached and its output is interleaved with the telnet session
	openocd ${OPENOCD_FLAGS} & ${OPENOCD_SCRIPT} ${RUST_BIN} && ${OPENOCD_KILL}

clean-prev-swd: kill-telnet kill-openocd

kill-openocd:
	pkill openocd || exit 0

kill-telnet:
	pkill telnet || exit 0

# todo don't make this default
nobindings: build-arduino deploy-arduino listen-serial

# todo make it so we don't have to clean here...
# (rustbindgen / cbindgen order dependency)
#
# - could potentially run cbindgen from build script
#   to get rid of command line dependency and enforce order / correctness ???
#all: clean cbindgen build-arduino deploy-arduino listen-serial

rust: build-rust print-size deploy-rust listen-serial

# todo wrangle rust.h
tiny: build-tiny-arduino deploy-tiny listen-serial

cp-rust: build-arduino-rust-lib
	cp ${RUST_H} ${TINY_DIR}

build-rust:
	cargo build --release

print-size:
	exec ${GET_SIZE_CMD}

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

# retry because this 'randomly' fails sometimes (timing issue?)
_deploy:
	stm32flash ${FLASH_FLAGS} || stm32flash ${FLASH_FLAGS}

listen-serial:
	socat stdio ${SERIAL_DEVICE}

${ARDUINO_OUT_DIR}:
	mkdir -p ${ARDUINO_OUT_DIR}

clean:
	rm -rf ${ARDUINO_OUT_DIR}/* ${RUST_H} ${RUST_BINDINGS} target/thumbv7m-none-eabi
