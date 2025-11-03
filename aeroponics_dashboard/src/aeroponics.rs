use std::fmt;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum SensorData {
    Numeric(f32),
    Boolean(bool),
}

pub struct Towers(Vec<Tower>);

pub struct Tower {
    id: u16,
    sensors: Sensors,
    actuators: Actuators,
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

pub struct Actuators {}

impl Towers {
    pub fn new() -> Self {
        Towers(Vec::new())
    }
    pub fn add_tower(&mut self, tower: Tower) {
        self.0.push(tower);
    }
    pub fn get_by_id_mut(&mut self, id: u16) -> Option<&mut Tower> {
        self.0.iter_mut().find(|tower| tower.id == id)
    }
}

impl fmt::Display for Towers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tower in &self.0 {
            writeln!(f, "Tower ID {}: {}", tower.id, tower.sensors)?;
        }
        Ok(())
    }
}

impl Tower {
    pub fn new(id: u16) -> Self {
        Tower {
            id,
            sensors: Sensors::new(),
            actuators: Actuators::new(),
        }
    }
    pub fn update_sensor(&mut self, sensor_name: SensorName, value: SensorData) {
        self.sensors.update_sensor(sensor_name, value);
    }
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

    pub fn update_sensor(&mut self, sensor_name: SensorName, value: SensorData) {
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

impl Actuators {
    pub fn new() -> Self {
        Actuators {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tower() {
        let mut towers = Towers::new();
        let tower1 = Tower::new(1);
        let tower2 = Tower::new(2);
        towers.add_tower(tower1);
        towers.add_tower(tower2);

        assert_eq!(towers.0.len(), 2);
        assert_eq!(towers.0[0].id, 1);
        assert_eq!(towers.0[1].id, 2);
    }

    #[test]
    fn test_get_by_id_mut() {
        let mut towers = Towers::new();
        let tower1 = Tower::new(1);
        let tower2 = Tower::new(2);
        towers.add_tower(tower1);
        towers.add_tower(tower2);
        let tower = towers.get_by_id_mut(2);
        assert!(tower.is_some());
        assert_eq!(tower.unwrap().id, 2);
    }

    #[test]
    fn test_update_sensors() {
        let mut tower = Tower::new(1);
        tower.update_sensor(SensorName::TemperatureLower, SensorData::Numeric(22.5));
        tower.update_sensor(SensorName::TemperatureUpper, SensorData::Numeric(23.5));
        tower.update_sensor(SensorName::HumidityLower, SensorData::Numeric(55.0));
        tower.update_sensor(SensorName::HumidityUpper, SensorData::Numeric(60.0));
        tower.update_sensor(SensorName::Pressure, SensorData::Numeric(1013.25));
        tower.update_sensor(SensorName::Ec, SensorData::Numeric(1.5));
        tower.update_sensor(SensorName::Ph, SensorData::Numeric(6.8));
        tower.update_sensor(SensorName::WaterLevel, SensorData::Numeric(75.0));
        tower.update_sensor(SensorName::PumpRelay, SensorData::Boolean(true));
        tower.update_sensor(SensorName::PumpSolenoid, SensorData::Boolean(false));

        assert_eq!(tower.sensors.temperature_lower, Some(22.5));
        assert_eq!(tower.sensors.temperature_upper, Some(23.5));
        assert_eq!(tower.sensors.humidity_lower, Some(55.0));
        assert_eq!(tower.sensors.humidity_upper, Some(60.0));
        assert_eq!(tower.sensors.pressure, Some(1013.25));
        assert_eq!(tower.sensors.ec, Some(1.5));
        assert_eq!(tower.sensors.ph, Some(6.8));
        assert_eq!(tower.sensors.water_level, Some(75.0));
        assert_eq!(tower.sensors.pump_relay, Some(true));
        assert_eq!(tower.sensors.pump_solenoid, Some(false));
    }
}
