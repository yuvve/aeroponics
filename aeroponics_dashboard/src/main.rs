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

fn update_towers(towers: &mut Towers, topic: &str, payload: &str) {
    if let Some((id, sensor_name, sensor_data)) = parse_topic(topic, payload) {
        if let Some(tower) = towers.get_by_id_mut(id) {
            tower.update_sensor(sensor_name, sensor_data);
        } else {
            let mut new_tower = Tower::new(id);
            new_tower.update_sensor(sensor_name, sensor_data);
            towers.add_tower(new_tower);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut mqttoptions = MqttOptions::new("rumqtt-async", args.broker_ip, args.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("tower/#", QoS::AtMostOnce).await.unwrap();

    let mut towers = Towers::new();

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let payload = String::from_utf8_lossy(&p.payload);
                update_towers(&mut towers, &p.topic, &payload);
                println!("{}", towers);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
}
