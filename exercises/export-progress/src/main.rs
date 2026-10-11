use export_progress::{ExportSection, format_event, start_export};

fn main() {
    let sections = vec![
        ExportSection {
            name: "orders".into(),
            rows: 3,
            fail: false,
        },
        ExportSection {
            name: "customers".into(),
            rows: 2,
            fail: false,
        },
    ];

    let (receiver, worker) = start_export(sections);
    for event in receiver {
        println!("{}", format_event(&event));
    }
    worker.join().expect("demo exporter should finish");
}
