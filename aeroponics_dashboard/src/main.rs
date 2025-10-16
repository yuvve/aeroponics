mod aeroponics;

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

fn parse_message(topic: &str, payload: &str) -> Option<(&str, f32)> {
    if let Ok(value) = payload.parse::<f32>() {
        Some((topic, value))
        // TODO: match topic and update sensors
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut sensor_data = Sensors::new_empty();
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
