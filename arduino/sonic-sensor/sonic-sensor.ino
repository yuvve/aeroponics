/* -*- mode: c++ -*- */
// Datasheet: https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf

const int trigger_input = A0;
const int echo = A1;


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
    delay(70); // > 60 ms measurement cycle
}
