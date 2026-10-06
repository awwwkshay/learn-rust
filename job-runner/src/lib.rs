use std::collections::VecDeque;

/// A unit of work that can be run by the scheduler.
pub trait Job {
    fn name(&self) -> &str;
    fn run(&self) -> Result<String, String>;
}

pub struct UppercaseJob {
    pub name: String,
    pub input: String,
}

impl Job for UppercaseJob {
    fn name(&self) -> &str {
        todo!()
    }

    fn run(&self) -> Result<String, String> {
        todo!()
    }
}

pub struct WordCountJob {
    pub name: String,
    pub input: String,
}

impl Job for WordCountJob {
    fn name(&self) -> &str {
        todo!()
    }

    fn run(&self) -> Result<String, String> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome {
    Completed(String),
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RunReport {
    pub name: String,
    pub outcome: RunOutcome,
}

pub struct Scheduler {
    pending: VecDeque<Box<dyn Job>>,
}

impl Scheduler {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn enqueue(&mut self, job: Box<dyn Job>) {
        todo!()
    }

    /// Runs one pending job, removing it from the queue even if it fails.
    pub fn run_next(&mut self) -> Option<RunReport> {
        todo!()
    }

    /// Drains the queue in FIFO order and keeps running after a failed job.
    pub fn run_all(&mut self) -> Vec<RunReport> {
        todo!()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Formats one report per line, without a trailing newline.
pub fn format_reports(reports: &[RunReport]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingJob;

    impl Job for FailingJob {
        fn name(&self) -> &str {
            "unavailable"
        }

        fn run(&self) -> Result<String, String> {
            Err("service offline".to_string())
        }
    }

    #[test]
    fn concrete_jobs_produce_results() {
        let upper = UppercaseJob {
            name: "headline".into(),
            input: "Ready for review".into(),
        };
        assert_eq!(upper.name(), "headline");
        assert_eq!(upper.run(), Ok("READY FOR REVIEW".into()));

        let count = WordCountJob {
            name: "word count".into(),
            input: "  one\n two   three ".into(),
        };
        assert_eq!(count.name(), "word count");
        assert_eq!(count.run(), Ok("3 words".into()));
    }

    #[test]
    fn empty_input_is_valid_work() {
        let count = WordCountJob {
            name: "empty".into(),
            input: "  \n ".into(),
        };
        assert_eq!(count.run(), Ok("0 words".into()));
    }

    #[test]
    fn empty_scheduler_has_no_next_job() {
        let mut scheduler = Scheduler::new();
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.len(), 0);
        assert_eq!(scheduler.run_next(), None);
        assert!(scheduler.run_all().is_empty());
    }

    #[test]
    fn runs_heterogeneous_jobs_in_order() {
        let mut scheduler = Scheduler::new();
        scheduler.enqueue(Box::new(UppercaseJob {
            name: "upper".into(),
            input: "hello".into(),
        }));
        scheduler.enqueue(Box::new(WordCountJob {
            name: "count".into(),
            input: "one two".into(),
        }));
        assert_eq!(scheduler.len(), 2);
        assert_eq!(
            scheduler.run_all(),
            vec![
                RunReport {
                    name: "upper".into(),
                    outcome: RunOutcome::Completed("HELLO".into())
                },
                RunReport {
                    name: "count".into(),
                    outcome: RunOutcome::Completed("2 words".into())
                },
            ]
        );
        assert!(scheduler.is_empty());
    }

    #[test]
    fn failure_does_not_stop_later_jobs() {
        let mut scheduler = Scheduler::new();
        scheduler.enqueue(Box::new(FailingJob));
        scheduler.enqueue(Box::new(UppercaseJob {
            name: "later".into(),
            input: "still runs".into(),
        }));
        assert_eq!(
            scheduler.run_next(),
            Some(RunReport {
                name: "unavailable".into(),
                outcome: RunOutcome::Failed("service offline".into()),
            })
        );
        assert_eq!(scheduler.len(), 1);
        assert_eq!(
            scheduler.run_next(),
            Some(RunReport {
                name: "later".into(),
                outcome: RunOutcome::Completed("STILL RUNS".into()),
            })
        );
    }

    #[test]
    fn formats_success_and_failure() {
        let reports = vec![
            RunReport {
                name: "upper".into(),
                outcome: RunOutcome::Completed("HELLO".into()),
            },
            RunReport {
                name: "unavailable".into(),
                outcome: RunOutcome::Failed("service offline".into()),
            },
        ];
        assert_eq!(
            format_reports(&reports),
            "upper: HELLO\nunavailable: ERROR service offline"
        );
        assert_eq!(format_reports(&[]), "");
    }
}
