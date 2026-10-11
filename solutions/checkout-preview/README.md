# Checkout preview

This is the completed first attempt, archived under `solutions/checkout-preview/`. The untouched starter remains under `exercises/checkout-preview/`.

## The situation

Before a customer confirms an order, a checkout page needs a current unit price and stock count. Those lookups are independent and may each wait on a different service. Build a small component that waits for both together and returns a useful preview or a specific reason it cannot make one. The supplied `SimulatedSource` stands in for a remote service; no network or credentials are needed.

Your parallel file audit used OS threads to do file work simultaneously. Here the main new idea is **concurrent waiting with async futures**. You will decide where to validate the request and how to combine two independent results without losing the distinction between a failed lookup and insufficient stock.

## Required behavior

- `SimulatedSource::fetch` asynchronously waits for its configured `delay`, then returns its configured `response`. `Err(())` represents a failed lookup. Use `tokio::time::sleep`; do not block the runtime thread with `std::thread::sleep`.
- `preview_order` rejects quantity zero with `InvalidQuantity` before polling either supplied future. For a positive quantity, poll the price and stock futures concurrently, and wait for **both** to finish even if one fails. Their completion order must not change the result.
- If price fails, return `PriceUnavailable`. Otherwise, if stock fails, return `StockUnavailable`. If both fail, price has priority. If both succeed but stock is less than the requested quantity, return `OutOfStock { available }`.
- On success, return the supplied quantity and price, the remaining stock, and `total_cents = unit_price_cents × quantity`. Convert to `u64` before multiplying. Values are in cents, so no floating point arithmetic is needed.

`preview_order` accepts futures rather than concrete service objects. That lets a caller supply real lookups later, while tests can supply local async blocks. An `async fn` produces a future that does no work until it is polled. `.await` drives one future to completion. [`tokio::join!`](https://docs.rs/tokio/latest/tokio/macro.join.html) polls both branches concurrently on one task and waits for both results; it does not run CPU work in parallel. `#[tokio::main]` runs the demo inside Tokio's runtime, and `#[tokio::test(start_paused = true)]` makes wait tests use a virtual clock. Tokio is the only dependency; the enabled `rt`, `macros`, and `time` features provide these APIs, while `test-util` enables the paused clock. See also [`sleep`](https://docs.rs/tokio/latest/tokio/time/fn.sleep.html) and [paused-time tests](https://docs.rs/tokio/latest/tokio/attr.test.html).

## Example and commands

From the repository root, run:

```sh
cargo test --manifest-path solutions/checkout-preview/Cargo.toml
cargo run --manifest-path solutions/checkout-preview/Cargo.toml
```

Or after `cd solutions/checkout-preview`, run:

```sh
cargo test
cargo run
```

`cargo run` prints exactly:

```text
2 items at 1250 cents each; total 2500 cents; 3 left
```

The tests cover a successful preview, zero quantity without polling sources, insufficient stock, each lookup failure, stable priority when both fail, and overlapping waits using Tokio's virtual clock. They call `preview_order` with independent test futures, so the two functions can be tested separately.

## Suggested order

1. Implement `preview_order` for a positive quantity and two successful results, then handle its error cases.
2. Check the virtual-clock test to confirm the two waits overlap.
3. Implement `SimulatedSource::fetch` and run the demo.

## Success criteria

`cargo test` passes and `cargo run` prints the line above. Be able to explain why two `.await`s in sequence would take longer, and why a price failure does not stop the stock future from finishing.

## Optional stretch goals

- Add a separate timeout rule for each lookup, preserving distinct price and stock timeout errors.
- Replace one simulated source with a caller-provided async function while keeping the preview logic independent of transport details.
