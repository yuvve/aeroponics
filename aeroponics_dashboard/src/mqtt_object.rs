pub trait MQTTObject<T> {
    fn from_mqtt(topic: &str, payload: &str) -> Option<T>;
    fn to_mqtt(&self) -> (String, String);
}
