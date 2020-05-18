MAIN_INO=      DRO-138.ino
OUTPUT_DIR=    ./build
OUTPUT_BIN=    ${OUTPUT_DIR}/${MAIN_INO}.bin
SERIAL_DEVICE= /dev/ttyUSB0

all: compile deploy

compile:
	mkdir -p ${OUTPUT_DIR} && arduino-builder -build-options-file build.options.json -build-path ${OUTPUT_DIR} ${MAIN_INO}

deploy: compile
	stm32flash -b 115200 -w ${OUTPUT_BIN} -v ${SERIAL_DEVICE}

clean:
	rm -rf build/*
