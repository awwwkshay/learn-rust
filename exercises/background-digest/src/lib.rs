use std::collections::BTreeMap;
use std::time::Duration;
use tokio::task::JoinHandle;

/// A completed frequency snapshot of all supplied messages.
#[derive(Debug, PartialEq, Eq)]
pub struct Digest {
    counts: BTreeMap<String, usize>,
}

impl Digest {
    /// Returns a case-insensitive count, or zero for an absent term.
    pub fn count(&self, term: &str) -> usize {
        let _ = term;
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DigestError {
    TaskFailed,
}

/// Builds a digest from borrowed messages, without starting a task.
pub fn build_digest(messages: &[String]) -> Digest {
    let _ = messages;
    todo!()
}

/// Launches one background task, transferring ownership of the messages to it.
pub fn start_digest(messages: Vec<String>, delay: Duration) -> JoinHandle<Digest> {
    let _ = (&messages, delay);
    todo!()
}

/// Waits for the task and turns a failed join into a domain error.
pub async fn finish_digest(handle: JoinHandle<Digest>) -> Result<Digest, DigestError> {
    let _ = &handle;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[test]
    fn builds_case_insensitive_counts_from_whitespace_separated_terms() {
        let messages = vec![
            "Rust async rust".to_string(),
            "  TOKIO\tRust.  ".to_string(),
            String::new(),
        ];
        let digest = build_digest(&messages);
        assert_eq!(
            digest.counts,
            BTreeMap::from([
                ("async".to_string(), 1),
                ("rust".to_string(), 2),
                ("rust.".to_string(), 1),
                ("tokio".to_string(), 1),
            ])
        );
        assert_eq!(messages[0], "Rust async rust");
    }

    #[test]
    fn empty_and_whitespace_only_messages_make_an_empty_digest() {
        assert_eq!(build_digest(&[]).counts, BTreeMap::new());
        assert_eq!(
            build_digest(&[" \t\n ".to_string(), String::new()]).counts,
            BTreeMap::new()
        );
    }

    #[test]
    fn count_queries_are_case_insensitive_and_missing_terms_are_zero() {
        let digest = Digest {
            counts: BTreeMap::from([("rust".to_string(), 3)]),
        };
        assert_eq!(digest.count("RUST"), 3);
        assert_eq!(digest.count("unknown"), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn task_runs_in_background_and_returns_its_digest() {
        let started = Instant::now();
        let handle = start_digest(vec!["Rust rust".to_string()], Duration::from_secs(5));
        assert_eq!(started.elapsed(), Duration::ZERO);

        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(!handle.is_finished());

        let digest = finish_digest(handle).await.unwrap();
        assert_eq!(digest.counts.get("rust"), Some(&2));
        assert!(started.elapsed() >= Duration::from_secs(5));
        assert!(started.elapsed() < Duration::from_secs(7));
    }

    #[tokio::test]
    async fn finish_accepts_a_completed_task() {
        let handle = tokio::spawn(async {
            Digest {
                counts: BTreeMap::from([("ready".to_string(), 1)]),
            }
        });
        let digest = finish_digest(handle).await.unwrap();
        assert_eq!(digest.counts.get("ready"), Some(&1));
    }

    #[tokio::test]
    async fn finish_reports_a_task_panic_without_panicking_itself() {
        let handle = tokio::spawn(async { panic!("simulated indexing failure") });
        assert_eq!(finish_digest(handle).await, Err(DigestError::TaskFailed));
    }
}
