use std::future::Future;
use std::time::Duration;

/// A local stand-in for one independent async service call.
pub struct SimulatedSource {
    pub delay: Duration,
    pub response: Result<u32, ()>,
}

impl SimulatedSource {
    /// Waits without blocking a thread, then returns the configured response.
    pub async fn fetch(&self) -> Result<u32, ()> {
        tokio::time::sleep(self.delay).await;
        return self.response;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OrderPreview {
    pub quantity: u32,
    pub unit_price_cents: u32,
    pub total_cents: u64,
    pub remaining_stock: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PreviewError {
    InvalidQuantity,
    PriceUnavailable,
    StockUnavailable,
    OutOfStock { available: u32 },
}

/// Polls independent price and stock futures concurrently and builds a preview.
pub async fn preview_order(
    quantity: u32,
    price: impl Future<Output = Result<u32, ()>>,
    stock: impl Future<Output = Result<u32, ()>>,
) -> Result<OrderPreview, PreviewError> {
    let _ = (quantity, &price, &stock);
    if quantity == 0 {
        return Err(PreviewError::InvalidQuantity);
    }
    let (price_result, stock_result) = tokio::join!(price, stock);
    let unit_price_cents = price_result.map_err(|_| PreviewError::PriceUnavailable)?;
    let available = stock_result.map_err(|_| PreviewError::StockUnavailable)?;

    if available < quantity {
        return Err(PreviewError::OutOfStock { available });
    }

    return Ok(OrderPreview {
        quantity: quantity,
        remaining_stock: available - quantity,
        unit_price_cents: unit_price_cents,
        total_cents: u64::from(unit_price_cents) * u64::from(quantity),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn simulated_source_waits_and_returns_its_response() {
        let source = SimulatedSource {
            delay: Duration::from_secs(3),
            response: Ok(42),
        };
        let started = Instant::now();
        assert_eq!(source.fetch().await, Ok(42));
        assert!(started.elapsed() >= Duration::from_secs(3));
        assert_eq!(
            SimulatedSource {
                delay: Duration::ZERO,
                response: Err(()),
            }
            .fetch()
            .await,
            Err(())
        );
    }

    #[tokio::test]
    async fn valid_order_combines_price_and_stock() {
        let preview = preview_order(2, async { Ok(1_250) }, async { Ok(5) })
            .await
            .unwrap();
        assert_eq!(
            preview,
            OrderPreview {
                quantity: 2,
                unit_price_cents: 1_250,
                total_cents: 2_500,
                remaining_stock: 3,
            }
        );
    }

    #[tokio::test]
    async fn zero_quantity_rejects_without_polling_sources() {
        let result = preview_order(0, async { panic!("price must not be polled") }, async {
            panic!("stock must not be polled")
        })
        .await;
        assert_eq!(result, Err(PreviewError::InvalidQuantity));
    }

    #[tokio::test]
    async fn insufficient_stock_reports_available_count() {
        assert_eq!(
            preview_order(4, async { Ok(500) }, async { Ok(3) }).await,
            Err(PreviewError::OutOfStock { available: 3 })
        );
    }

    #[tokio::test]
    async fn unavailable_sources_have_stable_error_priority() {
        assert_eq!(
            preview_order(1, async { Err(()) }, async { Ok(3) }).await,
            Err(PreviewError::PriceUnavailable)
        );
        assert_eq!(
            preview_order(1, async { Ok(500) }, async { Err(()) }).await,
            Err(PreviewError::StockUnavailable)
        );
        assert_eq!(
            preview_order(1, async { Err(()) }, async { Err(()) }).await,
            Err(PreviewError::PriceUnavailable)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn independent_waits_overlap() {
        let started = Instant::now();
        let preview = preview_order(
            2,
            async {
                tokio::time::sleep(Duration::from_secs(4)).await;
                Ok(100)
            },
            async {
                tokio::time::sleep(Duration::from_secs(7)).await;
                Ok(5)
            },
        )
        .await
        .unwrap();
        assert_eq!(preview.total_cents, 200);
        assert!(started.elapsed() >= Duration::from_secs(7));
        assert!(started.elapsed() < Duration::from_secs(9));
    }

    #[tokio::test(start_paused = true)]
    async fn failed_price_does_not_skip_the_stock_lookup() {
        let mut stock_finished = false;
        let result = preview_order(1, async { Err(()) }, async {
            tokio::time::sleep(Duration::from_secs(2)).await;
            stock_finished = true;
            Ok(5)
        })
        .await;
        assert_eq!(result, Err(PreviewError::PriceUnavailable));
        assert!(stock_finished);
    }
}
