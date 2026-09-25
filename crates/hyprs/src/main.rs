use hyprs::events::sync::HyprlandEvents;

fn main() {
    let mut events = HyprlandEvents::new().unwrap();
    while let Ok(events) = events.read() {
        for event in events {
            println!("{event:?}");
        }
    }
}
