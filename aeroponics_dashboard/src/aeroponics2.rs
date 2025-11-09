#[derive(Debug, PartialEq)]
pub enum SensorData {
    Numeric(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum ActuatorData {
    Boolean(bool),
}

pub trait Actuator {
    fn set_state(&mut self, state: ActuatorData);
    fn get_state(&self) -> ActuatorData;
    fn to_mqtt(&self) -> (String, String);
}

pub trait Sensor {
    fn read_data(&self) -> SensorData;
    fn update_sensor(&mut self, data: SensorData);
    fn from_mqtt(name: &str, payload: &str) -> Self;
}

pub trait Tower {
    fn get_name(&self) -> &str;
    fn get_sensors(&self) -> &Vec<Box<dyn Sensor>>;
    fn get_actuators(&self) -> &Vec<Box<dyn Actuator>>;
}

