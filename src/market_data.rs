use ibapi::{contracts::Contract, market_data::realtime::TickTypes};

pub async fn log_ticks_and_collect(
    client: &ibapi::client::Client,
    symbol: &'static str,
    min_ticks_to_collect:usize
) -> Result<(Vec<TickTypes>, ibapi::subscriptions::Subscription<TickTypes>), Box<dyn std::error::Error>> {
    let contract = Contract::stock(symbol);
    tracing::info!("subscribing to market data for {}", contract.symbol);

    let market_data = client
        .market_data(&contract, &[], false, false)
        .await?;

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

    tracing::info!("market data collection complete for symbol {}; collected {} ticks", symbol, tick_count);
    Ok((ticks, market_data))
}
