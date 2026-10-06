use message_pipeline::Pipeline;

fn main() {
    let mut pipeline = Pipeline::new();
    pipeline.add_step(|message| Ok(message.trim().to_string()));
    pipeline.add_step(|message| {
        if message.is_empty() {
            Err("empty message".to_string())
        } else {
            Ok(message)
        }
    });

    let mut accepted = 0;
    pipeline.add_step(move |message| {
        accepted += 1;
        if accepted > 2 {
            Err("limit reached".to_string())
        } else {
            Ok(message.to_uppercase())
        }
    });

    let messages = vec!["  hello  ", "   ", "Rust", "again"]
        .into_iter()
        .map(str::to_string)
        .collect();
    for (index, result) in pipeline.process_batch(messages).into_iter().enumerate() {
        match result {
            Ok(message) => println!("{}: {}", index + 1, message),
            Err(error) => println!("{}: ERROR {}", index + 1, error),
        }
    }
}
