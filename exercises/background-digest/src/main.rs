use background_digest::{finish_digest, start_digest};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handle = start_digest(
        vec!["Rust async Rust".into(), "Tokio awaits Rust".into()],
        Duration::from_millis(20),
    );
    println!("digest started");

    let digest = finish_digest(handle)
        .await
        .expect("demo task should succeed");
    println!("rust: {}", digest.count("rust"));
    println!("tokio: {}", digest.count("tokio"));
}
