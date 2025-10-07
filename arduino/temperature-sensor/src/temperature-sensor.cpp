#include <OneWire.h>
#include <DallasTemperature.h>

const int pin = 1;
OneWire myWire(pin);
DallasTemperature sensors(&myWire);
char buf[256];

void setup()
{
    Serial.begin(115200);
    sensors.begin();
}

void loop()
{
    sensors.requestTemperatures();
    delay(800); // Adjust this for update frequency (>750 ms)
    float temp = sensors.getTempCByIndex(0); // First IC on the bus
    if(temp == DEVICE_DISCONNECTED_C)
        Serial.println("NULL");
    sprintf(buf, "T:%f", temp);
    Serial.println(buf);
}
