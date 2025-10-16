use std::fmt;

type Towers = Vec<AeroponicsTower>;

struct AeroponicsTower {
    id: u16,
    sensors: Sensors,
    actuators: Actuators,
}

impl AeroponicsTower {
    fn new(id: u16) -> Self {
        AeroponicsTower {
            id,
            sensors: Sensors::new_empty(),
            actuators: Actuators::new(),
        }
    }
}

struct Sensors {
    temperature: f32,
    humidity: f32,
    pressure: f32,
    ec: f32,
    ph: f32,
    water_level: f32,
    pump_relay: bool,
    pump_solenoid: bool,
}

trait SensorsUpdate {
    fn update_sensors(&mut self);
}

struct Actuators {}

trait ActuatorControl {
    fn pump_relay(tower_id: u16, state: bool);
    fn pump_solenoid(tower_id: u16, state: bool);
}

impl Sensors {
    fn new(
        temperature: f32,
        humidity: f32,
        pressure: f32,
        ec: f32,
        ph: f32,
        water_level: f32,
        pump_relay: bool,
        pump_solenoid: bool,
    ) -> Self {
        Sensors {
            temperature,
            humidity,
            pressure,
            ec,
            ph,
            water_level,
            pump_relay,
            pump_solenoid,
        }
    }
    fn new_empty() -> Self {
        Sensors {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
            ec: 0.0,
            ph: 0.0,
            water_level: 0.0,
            pump_relay: false,
            pump_solenoid: false,
        }
    }
}

impl Actuators {
    fn new() -> Self {
        Actuators {}
    }
}

impl fmt::Display for Sensors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Temperature: {:.2} °C, Humidity: {:.2} %, Pressure: {:.2} hPa, EC: {:.2} µS/cm, pH: {:.2}, Water Level: {:.2} cm, Pump Relay: {}, Pump Solenoid: {}",
            self.temperature,
            self.humidity,
            self.pressure,
            self.ec,
            self.ph,
            self.water_level,
            self.pump_relay,
            self.pump_solenoid
        )
    }
}
