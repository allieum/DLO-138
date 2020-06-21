#include "rust.h";

void print_serial(const char* str) {
	Serial.print(str);
	delay(100); // wait enough time for the message to transmit in case we crash soon
}

// ------------------------
void setup()	{
// ------------------------
  // suggestion from https://github.com/ardyesp/DLO-138/issues/11#issuecomment-413483903
	//afio_cfg_debug_ports(AFIO_DEBUG_NONE);

		Serial.begin(115200);

		print_serial("tiny hi");

	tiny_init(print_serial);
}



// ------------------------
void loop()	{
// ------------------------
	print_serial("whee.");
}
