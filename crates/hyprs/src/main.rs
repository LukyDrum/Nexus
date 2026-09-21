use hyprs::events::tokio::HyprlandEvents;

#[tokio::main]
async fn main() {
    let mut events = HyprlandEvents::new().await.unwrap();
    while let Ok(events) = events.read().await {
        for event in events {
            println!("{event:?}");
        }
    }
}
