use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

pub async fn run_publisher() {
    // MQTT-Optionen für den Publisher
    let mut mqtt_options = MqttOptions::new("rust-publisher", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    // Erstelle den asynchronen MQTT-Client
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    // Nachricht veröffentlichen (publish)
    client
        .publish(
            "test/topic",
            QoS::AtLeastOnce,
            false,
            "Hello MQTT from Rust!",
        )
        .await
        .unwrap();

    println!("Nachricht wurde veröffentlicht!");

    // Lass den Eventloop laufen (notwendig für die korrekte Kommunikation)
    tokio::spawn(async move {
        loop {
            let _ = eventloop.poll().await;
        }
    });

    // Eine kurze Wartezeit, um sicherzustellen, dass die Nachricht gesendet wird
    tokio::time::sleep(Duration::from_secs(1)).await;
}
