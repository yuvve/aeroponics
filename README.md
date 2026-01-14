# aeroponics

## Project Structure

. \
├── arduino \
│   └── Sensor example code written with PlatformIO \
├── design \
│   └── Buck-converter PCB files\
├── docs \
│   ├── Diagrams \
│   ├── Failure mode and effect analysis  \
│   └── Bill of materials \
├── ESPHome \
│   ├── Readme with ESPHome instructions \
│   └── ESPHome code for the microcontrollers \
├── OpenHAB \
│   ├── OpenHAB configuration files \
│   ├── OpenHAB automation scripts \
│   └── OpenHAB UI definitions \
├── RPi \
│   ├── Mosquitto MQTT broker configuration files \
│   └── Mosquitto MQTT broker docker-compose \
└── Simulations \
    └── EC sensor SPICE simulation files for Cadence Capture CIS

## Notes

At the current state there is no EC-sensor that is done. However, there is a simulation file that has been made that is meant to be worked on so that it may one day meet specification for the rest of the project. Thease specifications include but are not limited to: being able to be powered by the given power source, not requireing negative source and have an output range between 0 - 3.3V as not to fry the microcontroller.
