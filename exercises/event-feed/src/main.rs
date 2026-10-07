use event_feed::{Event, EventFeed};

fn main() {
    let app = vec![
        Event {
            timestamp: 2,
            source: "app".into(),
            message: "started".into(),
        },
        Event {
            timestamp: 2,
            source: "app".into(),
            message: "ready".into(),
        },
        Event {
            timestamp: 5,
            source: "app".into(),
            message: "served request".into(),
        },
    ];
    let database = vec![
        Event {
            timestamp: 1,
            source: "db".into(),
            message: "connected".into(),
        },
        Event {
            timestamp: 2,
            source: "db".into(),
            message: "query finished".into(),
        },
    ];

    let feed = EventFeed::new(app, database).expect("demo inputs are sorted");
    for event in feed {
        println!("{} [{}] {}", event.timestamp, event.source, event.message);
    }
}
