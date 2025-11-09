#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum SensorData {
    Numeric(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ActuatorName {
    Pump,
    Solenoid,
}

#[derive(Debug, PartialEq)]
pub enum ActuatorData {
    Boolean(bool),
}

pub trait Towers {
    fn get_or_add_tower(&mut self, tower_id: u16) -> &mut dyn Tower;
    fn get_sensor(&self, tower_id: u16, sensor_name: SensorName) -> Option<&dyn Sensor>;
    fn get_actuator(&self, tower_id: u16, actuator_name: ActuatorName) -> Option<&dyn Actuator>;
    fn get_actuator_mqtt(
        &self,
        tower_id: u16,
        actuator_name: ActuatorName,
        actuator_data: ActuatorData,
    ) -> Option<String>;
}

pub struct TowersImpl {
    pub towers: std::collections::HashMap<u16, Box<dyn Tower>>,
}

impl TowersImpl {
    fn new() -> Self {
        TowersImpl {
            towers: std::collections::HashMap::new(),
        }
    }
}

impl Towers for TowersImpl {
    fn get_or_add_tower(&mut self, tower_id: u16) -> &mut dyn Tower {
        self.towers
            .entry(tower_id)
            .or_insert_with(|| {
                Box::new(TowerImpl {
                    id: tower_id,
                    sensors: std::collections::HashMap::new(),
                    actuators: std::collections::HashMap::new(),
                })
            })
            .as_mut()
    }

    fn get_sensor(&self, tower_id: u16, sensor_name: SensorName) -> Option<&dyn Sensor> {
        self.towers
            .get(&tower_id)
            .and_then(|tower| tower.get_sensor(sensor_name))
    }

    fn get_actuator(&self, tower_id: u16, actuator_name: ActuatorName) -> Option<&dyn Actuator> {
        self.towers
            .get(&tower_id)
            .and_then(|tower| tower.get_actuator(actuator_name))
    }

    fn get_actuator_mqtt(
        &self,
        tower_id: u16,
        actuator_name: ActuatorName,
        actuator_data: ActuatorData,
    ) -> Option<String> {
        self.towers
            .get(&tower_id)
            .and_then(|tower| tower.get_actuator_mqtt(actuator_name, actuator_data))
    }
}

pub trait Tower {
    fn get_sensor(&self, sensor_name: SensorName) -> Option<&dyn Sensor>;
    fn get_actuator(&self, actuator_name: ActuatorName) -> Option<&dyn Actuator>;
    fn get_actuator_mqtt(
        &self,
        actuator_name: ActuatorName,
        actuator_data: ActuatorData,
    ) -> Option<String>;
}

pub struct TowerImpl {
    pub id: u16,
    pub sensors: std::collections::HashMap<SensorName, Box<dyn Sensor>>,
    pub actuators: std::collections::HashMap<ActuatorName, Box<dyn Actuator>>,
}

impl TowerImpl {
    fn new(tower_id: u16) -> Self {
        TowerImpl {
            id: tower_id,
            sensors: std::collections::HashMap::new(),
            actuators: std::collections::HashMap::new(),
        }
    }
}

impl Tower for TowerImpl {
    fn get_sensor(&self, sensor_name: SensorName) -> Option<&dyn Sensor> {
        self.sensors.get(&sensor_name).map(|s| s.as_ref())
    }

    fn get_actuator(&self, actuator_name: ActuatorName) -> Option<&dyn Actuator> {
        self.actuators.get(&actuator_name).map(|a| a.as_ref())
    }

    fn get_actuator_mqtt(
        &self,
        actuator_name: ActuatorName,
        actuator_data: ActuatorData,
    ) -> Option<String> {
        self.actuators
            .get(&actuator_name)
            .and_then(|actuator| actuator.get_mqtt(actuator_data))
    }
}

pub trait Sensor {
    fn read_data(&self) -> SensorData;
    fn update_data(&mut self, sensor_data: SensorData);
}

pub trait Actuator {
    fn get_mqtt(&self, actuator_data: ActuatorData) -> Option<String>;
}
