use ibapi::{contracts::Contract, market_data::realtime::TickTypes};

use crate::core::{GuardedResult, SubscriptionGuard};

pub async fn log_ticks_and_collect(
    client: &ibapi::client::Client,
    symbol: &'static str,
    min_ticks_to_collect: usize,
) -> Result<GuardedResult<Vec<TickTypes>>, Box<dyn std::error::Error>> {
    let contract = Contract::stock(symbol);
    tracing::info!("subscribing to market data for {}", contract.symbol);

    let market_data = client.market_data(&contract, &[], false, false).await?;

    let subscription_guard = SubscriptionGuard::from(&market_data);

    let mut market_data = market_data;
    let mut tick_count = 0;

    let mut ticks: Vec<TickTypes> = Vec::default();

    while let Some(Ok(tick)) = market_data.next().await {
        tick_count += 1;

        if tick_count >= min_ticks_to_collect {
            break;
        }

        ticks.push(tick);
    }

    tracing::info!(
        "market data collection complete for symbol {}; collected {} ticks",
        symbol,
        tick_count
    );

    Ok(subscription_guard.to_guarded_result(ticks))
}
