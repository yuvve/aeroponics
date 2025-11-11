use mqtt_object::MQTTObject;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SensorName {
    TemperatureUpper,
    HumidityUpper,
    Pressure,
    Ec,
    Ph,
    WaterLevel,
    PumpPower,
    SolenoidPeriod,
    SolenoidDutyCycle,
}

impl MQTTObject for SensorName {
    /// Assumes topic is in the format "sensor/sensor-name"
    fn from_mqtt(topic: &str, _payload: &str) -> Option<SensorName> {
        if !topic.starts_with("sensor/") {
            return None;
        }
        let sensor_name = topic.split('/').last().unwrap_or("");
        match sensor_name {
            "temperature-upper" => Some(SensorName::TemperatureUpper),
            "humidity-upper" => Some(SensorName::HumidityUpper),
            "pressure" => Some(SensorName::Pressure),
            "ec" => Some(SensorName::Ec),
            "ph" => Some(SensorName::Ph),
            "water-level" => Some(SensorName::WaterLevel),
            "pump-power" => Some(SensorName::PumpPower),
            "solenoid-period" => Some(SensorName::SolenoidPeriod),
            "solenoid-duty-cycle" => Some(SensorName::SolenoidDutyCycle),
            _ => None,
        }
    }

    fn to_mqtt(&self) -> (String, String) {
        let topic = match self {
            SensorName::TemperatureUpper => "sensor/temperature-upper",
            SensorName::HumidityUpper => "sensor/humidity-upper",
            SensorName::Pressure => "sensor/pressure",
            SensorName::Ec => "sensor/ec",
            SensorName::Ph => "sensor/ph",
            SensorName::WaterLevel => "sensor/water-level",
            SensorName::PumpPower => "sensor/pump-power",
            SensorName::SolenoidPeriod => "sensor/solenoid-period",
            SensorName::SolenoidDutyCycle => "sensor/solenoid-duty-cycle",
        };
        (topic.to_string(), String::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SensorData {
    Numeric(f32),
    Boolean(bool),
}

impl MQTTObject for SensorData {
    fn from_mqtt(_topic: &str, payload: &str) -> Option<SensorData> {
        if let Ok(num) = payload.parse::<f32>() {
            Some(SensorData::Numeric(num))
        } else if let Ok(b) = payload.parse::<bool>() {
            Some(SensorData::Boolean(b))
        } else {
            None
        }
    }

    fn to_mqtt(&self) -> (String, String) {
        match self {
            SensorData::Numeric(num) => ("".to_string(), num.to_string()),
            SensorData::Boolean(b) => ("".to_string(), b.to_string()),
        }
    }
}

pub struct Sensor {
    name: SensorName,
    data: SensorData,
}

impl MQTTObject for Sensor {
    fn from_mqtt(topic: &str, payload: &str) -> Option<Sensor> {
        let name = SensorName::from_mqtt(topic, payload)?;
        let data = SensorData::from_mqtt(topic, payload)?;
        Some(Sensor { name, data })
    }

    fn to_mqtt(&self) -> (String, String) {
        let (topic, _) = self.name.to_mqtt();
        let (_, payload) = self.data.to_mqtt();
        (topic, payload)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ActuatorName {
    PumpPower,
    SolenoidPeriod,
    SolenoidDutyCycle,
}

impl MQTTObject for ActuatorName {
    fn from_mqtt(topic: &str, _payload: &str) -> Option<ActuatorName> {
        if !topic.starts_with("control/") {
            return None;
        }
        let actuator_name = topic.split('/').last().unwrap_or("");
        match actuator_name {
            "pump-power" => Some(ActuatorName::PumpPower),
            "solenoid-period" => Some(ActuatorName::SolenoidPeriod),
            "solenoid-duty-cycle" => Some(ActuatorName::SolenoidDutyCycle),
            _ => None,
        }
    }

    fn to_mqtt(&self) -> (String, String) {
        let topic = match self {
            ActuatorName::PumpPower => "pump-power",
            ActuatorName::SolenoidPeriod => "solenoid-period",
            ActuatorName::SolenoidDutyCycle => "solenoid-duty-cycle",
        };
        (topic.to_string(), String::new())
    }
}

#[derive(Debug, PartialEq)]
pub enum ActuatorData {
    Boolean(bool),
    Numeric(f32),
}

impl MQTTObject for ActuatorData {
    fn from_mqtt(_topic: &str, payload: &str) -> Option<ActuatorData> {
        if let Ok(b) = payload.parse::<bool>() {
            Some(ActuatorData::Boolean(b))
        } else if let Ok(num) = payload.parse::<f32>() {
            Some(ActuatorData::Numeric(num))
        } else {
            None
        }
    }

    fn to_mqtt(&self) -> (String, String) {
        match self {
            ActuatorData::Boolean(b) => ("".to_string(), b.to_string()),
            ActuatorData::Numeric(num) => ("".to_string(), num.to_string()),
        }
    }
}

pub struct Actuator {
    name: ActuatorName,
    data: ActuatorData,
}

impl MQTTObject for Actuator {
    fn from_mqtt(topic: &str, payload: &str) -> Option<Actuator> {
        let name = ActuatorName::from_mqtt(topic, payload)?;
        let data = ActuatorData::from_mqtt(topic, payload)?;
        Some(Actuator { name, data })
    }

    fn to_mqtt(&self) -> (String, String) {
        let (topic, _) = self.name.to_mqtt();
        let (_, payload) = self.data.to_mqtt();
        (topic, payload)
    }
}

pub struct Tower {
    id: u16,
    sensors: std::collections::HashMap<SensorName, SensorData>,
    actuators: std::collections::HashMap<ActuatorName, ActuatorData>,
}

impl Tower {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            sensors: std::collections::HashMap::new(),
            actuators: std::collections::HashMap::new(),
        }
    }
}

impl MQTTObject for Tower {
    fn from_mqtt(topic: &str, payload: &str) -> Option<Tower> {
        let parts: Vec<&str> = topic.split('/').collect();
        if parts.len() != 3 || parts[0] != "tower" {
            return None;
        }

        let id: u16 = parts[1].parse().ok()?;
        let name = parts[2];

        let mut tower = Tower::new(id);

        if let Some(sensor) = Sensor::from_mqtt(name, payload) {
            tower.sensors.insert(sensor.name, sensor.data);
        }

        Some(tower)
    }

    fn to_mqtt(&self) -> (String, String) {
        (format!("tower/{}", self.id), String::new())
    }
}

pub struct Towers {
    towers: std::collections::HashMap<u16, Tower>,
}

impl Towers {
    pub fn new() -> Self {
        Self {
            towers: std::collections::HashMap::new(),
        }
    }
    pub fn get_by_id_mut(&mut self, id: u16) -> Option<&mut Tower> {
        self.towers.get_mut(&id)
    }
    pub fn add_tower(&mut self, tower: Tower) {
        self.towers.insert(tower.id, tower);
    }
}
