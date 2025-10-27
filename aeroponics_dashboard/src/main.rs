mod aeroponics;

use aeroponics::*;
use clap::Parser;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tokio::{task, time};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    broker_ip: String,

    #[arg(short, long, default_value_t = 1883)]
    port: u16,
}

fn parse_topic(topic: &str, payload: &str) -> Option<(u16, SensorName, SensorData)> {
    let parts: Vec<_> = topic.split('/').collect();

    if parts.len() == 4 && parts[0] == "tower" && parts[2] == "sensor" {
        let id = parts[1].parse::<u16>().ok()?;
        let sensor_name = parts[3];
        match sensor_name {
            "temp-lower" => Some((
                id,
                SensorName::TemperatureLower,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "temp-upper" => Some((
                id,
                SensorName::TemperatureUpper,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "humidity-lower" => Some((
                id,
                SensorName::HumidityLower,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "humidity-upper" => Some((
                id,
                SensorName::HumidityUpper,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "pressure" => Some((
                id,
                SensorName::Pressure,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "ec" => Some((
                id,
                SensorName::Ec,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "ph" => Some((
                id,
                SensorName::Ph,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "water-level" => Some((
                id,
                SensorName::WaterLevel,
                SensorData::Numeric(payload.parse().ok()?),
            )),
            "pump" => Some((
                id,
                SensorName::PumpRelay,
                SensorData::Boolean(payload.parse().ok()?),
            )),
            "solenoid" => Some((
                id,
                SensorName::PumpSolenoid,
                SensorData::Boolean(payload.parse().ok()?),
            )),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_message(towers: &Towers, topic: &str, payload: &str) {
    if let Some((id, sensor_name, sensor_data)) = parse_topic(topic, payload) {
        // TODO: Update sensors
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut sensor_data = Sensors::new();
    let mut mqttoptions = MqttOptions::new("rumqtt-async", args.broker_ip, args.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("tower1/sensor/temperature", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let payload = String::from_utf8_lossy(&p.payload);
                match p.topic.as_str() {
                    "tower1/sensor/temperature" => {
                        if let Ok(temp) = payload.parse::<f32>() {
                            sensor_data.temperature = temp;
                        }
                    }
                    "tower1/sensor/humidity" => {
                        if let Ok(hum) = payload.parse::<f32>() {
                            sensor_data.humidity = hum;
                        }
                    }
                    "tower1/sensor/pressure" => {
                        if let Ok(pres) = payload.parse::<f32>() {
                            sensor_data.pressure = pres;
                        }
                    }
                    "tower1/sensor/ec" => {
                        if let Ok(ec) = payload.parse::<f32>() {
                            sensor_data.ec = ec;
                        }
                    }
                    "tower1/sensor/ph" => {
                        if let Ok(ph) = payload.parse::<f32>() {
                            sensor_data.ph = ph;
                        }
                    }
                    "tower1/sensor/water_level" => {
                        if let Ok(wl) = payload.parse::<f32>() {
                            sensor_data.water_level = wl;
                        }
                    }
                    _ => {}
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
        print!("\r{}", sensor_data);
    }
}
