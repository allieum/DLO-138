
enum { TRIGGER_AUTO, TRIGGER_NORM, TRIGGER_SINGLE };
uint8_t triggerType;


// ------------------------
void setTriggerType(uint8_t tType)	{
// ------------------------
	triggerType = tType;
	// break any running capture loop
	keepSampling = false;
}

extern Adafruit_TFTLCD_8bit_STM32 tft;

// ------------------------
void controlLoop()	{
// ------------------------
	// start by reading the state of analog system
	// todo look into this one
	readInpSwitches();

	if(triggerType == TRIGGER_AUTO)	{
		captureDisplayCycle(true);
	}

	else if(triggerType == TRIGGER_NORM)	{
		captureDisplayCycle(false);
	}

	else	{
		// single trigger
		clearWaves();
		indicateCapturing();
		// blocking call - until trigger
		sampleWaves(false);
		indicateCapturingDone();
		hold = true;
		// request repainting of screen labels in next draw cycle
		repaintLabels();
		// draw the waveform
		draw_waves();
		blinkLED();
		// dump captured data on serial port
		dumpSamples();

		// freeze display
		//while(hold);

		// update display indicating hold released
		// drawLabels();
	}

	// process any long pending operations which cannot be serviced in ISR
}




// ------------------------
void captureDisplayCycle(boolean wTimeOut)	{
// ------------------------
	/* indicateCapturing(); */
	/* // blocking call - until timeout or trigger */
	/* sampleWaves(wTimeOut); */
	/* // draw the waveform */
	/* indicateCapturingDone(); */

	sample_wave();
	draw_waves();

	// inter wait before next sampling
	if(triggered)
		blinkLED();

	if(hold)	{
		// update UI labels
		// drawLabels();
		// dump captured data on serial port
		dumpSamples();
	}

	// freeze display if requested
	while(hold);
}
