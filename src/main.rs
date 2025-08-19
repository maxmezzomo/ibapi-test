use tracing_subscriber::EnvFilter;

mod market_data;

const IB_ADDRESS: &str = "127.0.0.1:7496";
const TEST_TIMEOUT_MS: u64 = 10000;
const TEST_HEARTBEAT_INTERVAL_MS: u64 = 1000;
const TICK_COUNT: usize = 3;
const SYMBOLS: &[&str] = &["AAPL", "AMZN"];

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .init();

    let client = ibapi::client::Client::connect(IB_ADDRESS, 100)
        .await
        .expect("connect should work for example");

    // I use delayed since paper account
    client
        .switch_market_data_type(ibapi::prelude::MarketDataType::Delayed)
        .await
        .unwrap();

    let futures = SYMBOLS
        .into_iter()
        .map(async |symbol| market_data::log_ticks_and_collect(&client, symbol, TICK_COUNT).await)
        .collect::<Vec<_>>();

    let collect_and_display_results = async {
        tracing::info!("starting futures join");
        let results = futures::future::join_all(futures).await;
        tracing::info!("futures join finishes with results: {:?}", results);
    };

    let test_timeout = tokio::time::sleep(std::time::Duration::from_millis(TEST_TIMEOUT_MS));

    let test_heartbeat = async {
        let mut heartbeat =
            tokio::time::interval(std::time::Duration::from_millis(TEST_HEARTBEAT_INTERVAL_MS));
        loop {
            heartbeat.tick().await;
            tracing::info!("waiting for test completion...")
        }
    };

    tokio::select! {
        _ = collect_and_display_results=>{
            tracing::info!("successfully completed collecting results")
        }

        _ = test_heartbeat => {}

        _ = test_timeout =>{
            tracing::warn!("test timeout")
        }
    }
}
