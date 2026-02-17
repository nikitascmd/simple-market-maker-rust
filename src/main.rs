mod exchanges;
mod localorderbook;
mod shared_state;

use anyhow::{Context, Result};
use binance_sdk::config::{ConfigurationRestApi, ConfigurationWebsocketStreams};
use binance_sdk::spot::SpotWsStreams;
use binance_sdk::spot::rest_api::{DepthParams, RestApi};
use binance_sdk::spot::websocket_streams::{BookTickerParams, DiffBookDepthParams, TradeParams};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // --- REST: fetch orderbook snapshot ---
    // We use RestApi::new() directly instead of SpotRestApi::production()
    // to avoid a bug where the SDK calls logger::init() twice and panics.
    let mut rest_config = ConfigurationRestApi::builder().build()?;
    rest_config.base_path = Some("https://api.binance.com".to_string());
    let client = RestApi::new(rest_config);

    let params = DepthParams::builder("BTCUSDT".to_string())
        .limit(100)
        .build()?;

    let response = client.depth(params).await?;
    let data = response.data().await?;

    println!("Bids: {:#?}", data.bids);
    println!("Asks: {:#?}", data.asks);

    // --- WebSocket Streams: listen to live data ---
    let ws_config = ConfigurationWebsocketStreams::builder().build()?;
    let ws_client = SpotWsStreams::production(ws_config);

    let connection = ws_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    let symbol = "btcusdt".to_string();

    // Subscribe to book ticker (best bid/ask)
    let bba_params = BookTickerParams::builder(symbol.clone()).build()?;
    let bba_stream = connection
        .book_ticker(bba_params)
        .await
        .context("Failed to subscribe to book ticker")?;
    bba_stream.on_message(|data| {
        info!("[BookTicker] {:?}", data);
    });

    // Subscribe to orderbook depth diffs
    let depth_params = DiffBookDepthParams::builder(symbol.clone()).build()?;
    let depth_stream = connection
        .diff_book_depth(depth_params)
        .await
        .context("Failed to subscribe to depth stream")?;
    depth_stream.on_message(|data| {
        info!("[Depth] {:?}", data);
    });

    // Subscribe to trades
    let trade_params = TradeParams::builder(symbol.clone()).build()?;
    let trade_stream = connection
        .trade(trade_params)
        .await
        .context("Failed to subscribe to trade stream")?;
    trade_stream.on_message(|data| {
        info!("[Trade] {:?}", data);
    });

    println!("Listening to streams... Press Ctrl+C to stop.");
    signal::ctrl_c().await?;

    connection
        .disconnect()
        .await
        .context("Failed to disconnect")?;

    println!("Disconnected.");
    Ok(())
}
