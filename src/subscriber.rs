use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

pub async fn run_subscriber() {
    // MQTT-Optionen für den Subscriber
    let mut mqtt_options = MqttOptions::new("rust-subscriber", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    // Erstelle den asynchronen MQTT-Client
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    // Abonniere das Topic (subscribe)
    client
        .subscribe("test/topic", QoS::AtLeastOnce)
        .await
        .unwrap();

    println!("Subscriber ist bereit und wartet auf Nachrichten...");

    // Starte den Eventloop und empfange Nachrichten
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    println!(
                        "Empfangene Nachricht auf {}: {}",
                        publish.topic,
                        String::from_utf8(publish.payload.to_vec()).unwrap()
                    );
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Fehler beim Empfang der Nachricht: {:?}", e);
                    break;
                }
            }
        }
    });

    // Lasse die Tokio-Runtime laufen
    tokio::time::sleep(Duration::from_secs(3600)).await;
}
