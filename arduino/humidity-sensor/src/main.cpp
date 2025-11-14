#include "Arduino.h"

const int pin = A0;
volatile unsigned long counter = 0;
unsigned long prev_t = 0;
float freq = 0.0;
const int sample_t = 1000; // ms
char buffer[256];

void countPulses() // For use in interrupt
{
	counter++;
}

void setup()
{
	Serial.begin(9600);
	pinMode(pin, INPUT);
	attachInterrupt(digitalPinToInterrupt(pin), countPulses, RISING);
}

void loop()
{
	unsigned long t = millis();
	if(t - prev_t >= sample_t)
	{
		freq = counter/(t-prev_t)*1e3;
		counter = 0;
		prev_t = t;
		sprintf(buffer, "Frequency: %f\r\n", freq);
		Serial.print(buffer);
	}
}
