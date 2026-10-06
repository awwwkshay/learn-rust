type Step = Box<dyn FnMut(String) -> Result<String, String>>;

/// Applies registered transformations and checks to each message in order.
pub struct Pipeline {
    steps: Vec<Step>,
}

impl Pipeline {
    pub fn new() -> Self {
        todo!()
    }

    pub fn step_count(&self) -> usize {
        todo!()
    }

    /// Stores a closure that may mutate its captured state between messages.
    pub fn add_step<F>(&mut self, step: F)
    where
        F: FnMut(String) -> Result<String, String> + 'static,
    {
        todo!()
    }

    /// Stops at the first error for this message; earlier step state is retained.
    pub fn process(&mut self, message: String) -> Result<String, String> {
        todo!()
    }

    /// Processes every message in order, including those after a failure.
    pub fn process_batch(&mut self, messages: Vec<String>) -> Vec<Result<String, String>> {
        todo!()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pipeline_has_no_steps() {
        let pipeline = Pipeline::new();
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn empty_pipeline_passes_message_through() {
        let mut pipeline = Pipeline { steps: Vec::new() };
        assert_eq!(pipeline.process("hello".into()), Ok("hello".into()));
    }

    #[test]
    fn registers_different_closure_types() {
        let mut pipeline = Pipeline { steps: Vec::new() };
        let suffix = String::from("!");
        pipeline.add_step(|message| Ok(message.trim().to_string()));
        pipeline.add_step(move |message| Ok(format!("{message}{suffix}")));
        assert_eq!(pipeline.step_count(), 2);
    }

    #[test]
    fn applies_steps_in_registration_order() {
        let mut pipeline = Pipeline {
            steps: vec![
                Box::new(|message: String| Ok(message.trim().to_string())),
                Box::new(|message: String| Ok(format!("{message}!"))),
            ],
        };
        assert_eq!(pipeline.process("  hello  ".into()), Ok("hello!".into()));
    }

    #[test]
    fn error_skips_later_steps_for_that_message() {
        let mut pipeline = Pipeline {
            steps: vec![
                Box::new(|_message: String| Err("blocked".to_string())),
                Box::new(|_message: String| panic!("later step must not run")),
            ],
        };
        assert_eq!(pipeline.process("hello".into()), Err("blocked".into()));
    }

    #[test]
    fn batch_continues_after_failure_and_keeps_closure_state() {
        let mut seen = 0;
        let mut pipeline = Pipeline {
            steps: vec![Box::new(move |message: String| {
                seen += 1;
                if seen == 2 {
                    Err("second message blocked".to_string())
                } else {
                    Ok(message)
                }
            })],
        };

        assert_eq!(
            pipeline.process_batch(vec!["one".into(), "two".into(), "three".into()]),
            vec![
                Ok("one".into()),
                Err("second message blocked".into()),
                Ok("three".into()),
            ]
        );
        assert_eq!(pipeline.process("four".into()), Ok("four".into()));
    }

    #[test]
    fn empty_batch_returns_no_results() {
        let mut pipeline = Pipeline { steps: Vec::new() };
        assert!(pipeline.process_batch(Vec::new()).is_empty());
    }
}
