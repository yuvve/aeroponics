mod aeroponics;

use aeroponics::*;
use clap::Parser;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    sync::mpsc,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    broker_ip: String,

    #[arg(short, long, default_value_t = 1883)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut mqttoptions = MqttOptions::new("rumqtt-async", args.broker_ip, args.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("tower/#", QoS::AtMostOnce).await.unwrap();

    client
        .publish("dashboard/hello", QoS::AtLeastOnce, false, "")
        .await
        .unwrap();

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(10);

    let mut towers = Towers::new();

    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    let payload = String::from_utf8_lossy(&p.payload);
                    towers.update(&p.topic, &payload);
                    println!("{}", towers);
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    break;
                }
            }
        }
    });

    let cmd_tx2 = cmd_tx.clone();
    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = cmd_tx2.send(line).await;
        }
    });

    while let Some(cmd) = cmd_rx.recv().await {
        println!("User typed: {}", cmd);
        handle_command(&cmd, &client).await;
    }
}

/// Command format: 'set <tower_id> <actuator> <state>'
/// Example: 'set 1 pump on'
/// Publishes to topic 'tower/<tower_id>/control/<actuator>' with payload '<state>'
/// Example topic: 'tower/1/control/pump' payload: 'on'
async fn handle_command(towers: &Towers, cmd: &str, client: &AsyncClient) {
    let cmd_lower = cmd.to_lowercase();
    let parts: Vec<&str> = cmd_lower.split_whitespace().collect();
    if parts.len() == 4 && parts[0] == "set" {
        let tower_id = parts[1];
        let actuator = parts[2];
        let state = parts[3];

        if let Some(tower) = towers.get_by_id_mut(tower_id.parse().unwrap()) {
            // TODO
        } else {
            println!("Tower with ID {} not found.", tower_id);
            return;
        }
        match client
            .publish(topic.clone(), QoS::AtLeastOnce, false, payload)
            .await
        {
            Ok(_) => println!("Published command to {}: {}", topic, payload),
            Err(e) => eprintln!("Failed to publish command: {:?}", e),
        }
    } else {
        println!("Unknown command format. Use: set <tower_id> <actuator> <state>");
    }
}
