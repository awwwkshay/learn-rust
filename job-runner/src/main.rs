use job_runner::{Scheduler, UppercaseJob, WordCountJob, format_reports};

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.enqueue(Box::new(UppercaseJob {
        name: "headline".into(),
        input: "Ready for review".into(),
    }));
    scheduler.enqueue(Box::new(WordCountJob {
        name: "word count".into(),
        input: "Rust jobs run in order".into(),
    }));
    println!("{}", format_reports(&scheduler.run_all()));
}
