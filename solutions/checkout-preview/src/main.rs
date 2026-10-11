use checkout_preview::{SimulatedSource, preview_order};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let price = SimulatedSource {
        delay: Duration::from_millis(30),
        response: Ok(1_250),
    };
    let stock = SimulatedSource {
        delay: Duration::from_millis(50),
        response: Ok(5),
    };

    let preview = preview_order(2, price.fetch(), stock.fetch())
        .await
        .expect("demo sources should provide an available item");
    println!(
        "{} items at {} cents each; total {} cents; {} left",
        preview.quantity, preview.unit_price_cents, preview.total_cents, preview.remaining_stock
    );
}
