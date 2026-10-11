use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

#[derive(Debug, PartialEq, Eq)]
pub struct ExportSection {
    pub name: String,
    pub rows: u32,
    pub fail: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportEvent {
    Started { sections: usize },
    SectionDone { name: String, total_rows: u64 },
    Failed { name: String, total_rows: u64 },
    Finished { total_rows: u64 },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportError {
    WorkerPanicked,
}

/// Starts one producer thread and returns its event receiver and join handle.
pub fn start_export(sections: Vec<ExportSection>) -> (Receiver<ExportEvent>, JoinHandle<()>) {
    let _ = sections;
    todo!()
}

/// Receives events until the channel closes, then joins the producer thread.
pub fn finish_export(
    receiver: Receiver<ExportEvent>,
    worker: JoinHandle<()>,
) -> Result<Vec<ExportEvent>, ExportError> {
    let _ = (&receiver, &worker);
    todo!()
}

/// Formats one event for an operator. The returned string has no newline.
pub fn format_event(event: &ExportEvent) -> String {
    let _ = event;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn section(name: &str, rows: u32, fail: bool) -> ExportSection {
        ExportSection {
            name: name.into(),
            rows,
            fail,
        }
    }

    #[test]
    fn formats_each_event_without_a_newline() {
        assert_eq!(
            format_event(&ExportEvent::Started { sections: 2 }),
            "starting 2 sections"
        );
        assert_eq!(
            format_event(&ExportEvent::SectionDone {
                name: "orders".into(),
                total_rows: 3,
            }),
            "done orders: 3 rows total"
        );
        assert_eq!(
            format_event(&ExportEvent::Failed {
                name: "payments".into(),
                total_rows: 3,
            }),
            "failed payments after 3 rows"
        );
        assert_eq!(
            format_event(&ExportEvent::Finished { total_rows: 3 }),
            "finished: 3 rows"
        );
    }

    #[test]
    fn finish_drains_a_supplied_channel_in_send_order() {
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            sender.send(ExportEvent::Started { sections: 1 }).unwrap();
            sender
                .send(ExportEvent::Finished { total_rows: 0 })
                .unwrap();
        });
        assert_eq!(
            finish_export(receiver, worker),
            Ok(vec![
                ExportEvent::Started { sections: 1 },
                ExportEvent::Finished { total_rows: 0 },
            ])
        );
    }

    #[test]
    fn finish_reports_a_panicking_worker() {
        let (sender, receiver) = mpsc::channel::<ExportEvent>();
        drop(sender);
        let worker = std::thread::spawn(|| panic!("simulated exporter crash"));
        assert_eq!(
            finish_export(receiver, worker),
            Err(ExportError::WorkerPanicked)
        );
    }

    #[test]
    fn successful_export_reports_progress_and_final_total() {
        let (receiver, worker) = start_export(vec![
            section("orders", 3, false),
            section("customers", 2, false),
        ]);
        assert_eq!(
            finish_export(receiver, worker),
            Ok(vec![
                ExportEvent::Started { sections: 2 },
                ExportEvent::SectionDone {
                    name: "orders".into(),
                    total_rows: 3,
                },
                ExportEvent::SectionDone {
                    name: "customers".into(),
                    total_rows: 5,
                },
                ExportEvent::Finished { total_rows: 5 },
            ])
        );
    }

    #[test]
    fn failed_section_stops_later_sections_and_keeps_partial_total() {
        let (receiver, worker) = start_export(vec![
            section("orders", 3, false),
            section("payments", 9, true),
            section("customers", 2, false),
        ]);
        assert_eq!(
            finish_export(receiver, worker),
            Ok(vec![
                ExportEvent::Started { sections: 3 },
                ExportEvent::SectionDone {
                    name: "orders".into(),
                    total_rows: 3,
                },
                ExportEvent::Failed {
                    name: "payments".into(),
                    total_rows: 3,
                },
            ])
        );
    }

    #[test]
    fn empty_export_still_has_a_start_and_finish_event() {
        let (receiver, worker) = start_export(Vec::new());
        assert_eq!(
            finish_export(receiver, worker),
            Ok(vec![
                ExportEvent::Started { sections: 0 },
                ExportEvent::Finished { total_rows: 0 },
            ])
        );
    }

    #[test]
    fn zero_row_section_is_reported_without_increasing_total() {
        let (receiver, worker) = start_export(vec![section("empty", 0, false)]);
        assert_eq!(
            finish_export(receiver, worker),
            Ok(vec![
                ExportEvent::Started { sections: 1 },
                ExportEvent::SectionDone {
                    name: "empty".into(),
                    total_rows: 0,
                },
                ExportEvent::Finished { total_rows: 0 },
            ])
        );
    }
}
