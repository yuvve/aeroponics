use std::fmt;

pub struct Towers(Vec<AeroponicsTower>);

impl Towers {
    fn new() -> Self {
        Towers(Vec::new())
    }
    fn add_tower(&mut self, id: u16) {
        self.0.push(AeroponicsTower::new(id));
    }
    fn get_by_id_mut(&mut self, id: u16) -> Option<&mut AeroponicsTower> {
        self.0.iter_mut().find(|tower| tower.id == id)
    }
}

struct AeroponicsTower {
    id: u16,
    sensors: Sensors,
    actuators: Actuators,
}

impl AeroponicsTower {
    fn new(id: u16) -> Self {
        AeroponicsTower {
            id,
            sensors: Sensors::new(),
            actuators: Actuators::new(),
        }
    }
}

pub struct Sensors {
    temperature_lower: Option<f32>,
    temperature_upper: Option<f32>,
    humidity_lower: Option<f32>,
    humidity_upper: Option<f32>,
    pressure: Option<f32>,
    ec: Option<f32>,
    ph: Option<f32>,
    water_level: Option<f32>,
    pump_relay: Option<bool>,
    pump_solenoid: Option<bool>,
}

impl Sensors {
    pub fn new() -> Self {
        Sensors {
            temperature_lower: None,
            temperature_upper: None,
            humidity_lower: None,
            humidity_upper: None,
            pressure: None,
            ec: None,
            ph: None,
            water_level: None,
            pump_relay: None,
            pump_solenoid: None,
        }
    }
}

pub enum SensorName {
    TemperatureLower,
    TemperatureUpper,
    HumidityLower,
    HumidityUpper,
    Pressure,
    Ec,
    Ph,
    WaterLevel,
    PumpRelay,
    PumpSolenoid,
}

pub enum SensorData {
    Numeric(f32),
    Boolean(bool),
}

impl From<SensorData> for f32 {
    fn from(data: SensorData) -> Self {
        match data {
            SensorData::Numeric(val) => val,
            _ => panic!("Expected numeric sensor data"),
        }
    }
}

impl From<SensorData> for bool {
    fn from(data: SensorData) -> Self {
        match data {
            SensorData::Boolean(val) => val,
            _ => panic!("Expected boolean sensor data"),
        }
    }
}

pub trait SensorUpdate {
    fn update_sensor(&mut self, sensor_name: SensorName, value: SensorData);
}

impl SensorUpdate for Sensors {
    fn update_sensor(&mut self, sensor_name: SensorName, value: SensorData) {
        match sensor_name {
            SensorName::TemperatureLower => self.temperature_lower = Some(value.into()),
            SensorName::TemperatureUpper => self.temperature_upper = Some(value.into()),
            SensorName::HumidityLower => self.humidity_lower = Some(value.into()),
            SensorName::HumidityUpper => self.humidity_upper = Some(value.into()),
            SensorName::Pressure => self.pressure = Some(value.into()),
            SensorName::Ec => self.ec = Some(value.into()),
            SensorName::Ph => self.ph = Some(value.into()),
            SensorName::WaterLevel => self.water_level = Some(value.into()),
            SensorName::PumpRelay => self.pump_relay = Some(value.into()),
            SensorName::PumpSolenoid => self.pump_solenoid = Some(value.into()),
            _ => {}
        }
    }
}

impl fmt::Display for Sensors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_opt<T: fmt::Display>(val: &Option<T>) -> String {
            val.as_ref()
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "N/A".to_string())
        }

        macro_rules! on_off {
            ($val:expr) => {
                match $val {
                    Some(true) => "ON",
                    Some(false) => "OFF",
                    None => "N/A",
                }
            };
        }

        write!(
            f,
            "Temp Lower: {}, Temp Upper: {}, Humidity Lower: {}, Humidity Upper: {}, \
             Pressure: {}, EC: {}, pH: {}, Water Level: {}, Pump Relay: {}, Pump Solenoid: {}",
            fmt_opt(&self.temperature_lower),
            fmt_opt(&self.temperature_upper),
            fmt_opt(&self.humidity_lower),
            fmt_opt(&self.humidity_upper),
            fmt_opt(&self.pressure),
            fmt_opt(&self.ec),
            fmt_opt(&self.ph),
            fmt_opt(&self.water_level),
            on_off!(self.pump_relay),
            on_off!(self.pump_solenoid)
        )
    }
}

pub struct Actuators {}

impl Actuators {
    pub fn new() -> Self {
        Actuators {}
    }
}

pub trait ActuatorControl {
    fn set_actuator(&mut self, actuator_name: SensorName, value: bool);
}
