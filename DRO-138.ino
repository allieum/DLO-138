#include <EEPROM.h>
#include <Adafruit_GFX.h>
#include "src/TFTLib/Adafruit_TFTLCD_8bit_STM32.h"
#include "global.h"
#include "variables.h"
#include "dro138.h"

#define FIRMWARE_VERSION	"1.0"

void girl(uint16_t snack) {

}

// ------------------------
void setup()	{
// ------------------------

        // suggestion from https://github.com/ardyesp/DLO-138/issues/11#issuecomment-413483903
        afio_cfg_debug_ports(AFIO_DEBUG_NONE);
	DBG_INIT(SERIAL_BAUD_RATE);
	DBG_PRINT("Dual channel O Scope with two logic channels, ver: ");
	DBG_PRINTLN(FIRMWARE_VERSION);

	// set digital and analog stuff
	initIO();

	// load scope config or factory reset to defaults
	loadConfig(digitalRead(BTN4) == LOW);

	// init the IL9341 display
	initDisplay();

	// chomp !
	girl(candy());
}



// ------------------------
void loop()	{
// ------------------------
	controlLoop();
}
