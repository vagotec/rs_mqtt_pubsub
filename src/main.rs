mod publisher;
mod subscriber;

#[tokio::main]
async fn main() {
    let mode = std::env::args()
        .nth(1)
        .expect("Mode (pub/sub) not provided");

    match mode.as_str() {
        "pub" => {
            println!("Starte Publisher...");
            publisher::run_publisher().await;
        }
        "sub" => {
            println!("Starte Subscriber...");
            subscriber::run_subscriber().await;
        }
        _ => {
            println!("Unbekannter Modus. Verwende 'pub' für Publisher oder 'sub' für Subscriber.");
        }
    }
}
