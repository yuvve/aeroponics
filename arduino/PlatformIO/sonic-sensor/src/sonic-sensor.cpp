#include <Arduino.h>

const int trigger_input = 2;
const int echo = 3;
char buf[256];
unsigned long distance;


void setup()
{
    pinMode(trigger_input, OUTPUT);
    pinMode(echo, INPUT);
    Serial.begin(115200);
}

void loop()
{
    digitalWrite(trigger_input, HIGH);
    delayMicroseconds(10);
    digitalWrite(trigger_input, LOW);

    distance = pulseIn(echo, HIGH)/58; // cm
    sprintf(buf, "Distance: %u\r\n", distance);
    Serial.write(buf);
    delay(70); // > 60 ms measurement cycle
}
