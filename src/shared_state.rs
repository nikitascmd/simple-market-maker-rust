use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::{env, fs};

use crate::exchanges::binance::websockets::handlers::orderbook::OrderBookBinance;
use serde::Deserialize;

const PARAMS_PATH: &str = "parameters.yaml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub account_size: f64,
    pub primary_data_feed: String,
    pub binance_symbol: String,
    pub bybit_symbol: String,
    pub price_offset: f64,
    pub size_offset: f64,
    pub volatility_offset: f64,
    pub base_spread: f64,
    pub min_order_size: f64,
    pub max_order_size: f64,
    pub inventory_extreme: f64,
    pub bollinger_band_length: u32,
    pub bollinger_band_std: f64,
}

#[derive(Debug)]
pub struct Secrets {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Copy)]
pub struct BestBidAsk {
    pub bid_price: f64,
    pub bid_qty: f64,
    pub ask_price: f64,
    pub ask_qty: f64,
}

#[derive(Debug)]
pub struct SharedState {
    pub config: Config,
    pub secrets: Secrets,
    binance_ws_connected: bool,
    binance_trades: AllocRingBuffer<f64>,
    binance_bba: BestBidAsk,
    binance_orderbook: OrderBookBinance,
    binance_last_price: f64,
    bybit_ws_connected: bool,
    bybit_klines: AllocRingBuffer<f64>,
    bybit_trades: AllocRingBuffer<f64>,
    bybit_bba: BestBidAsk,
}

impl SharedState {
    pub fn new() -> SharedState {
        let contents =
            fs::read_to_string(PARAMS_PATH).expect(&format!("Failed to parse {}", PARAMS_PATH));

        let config: Config =
            serde_yaml::from_str(&contents).expect(&format!("Failed to parse {}", PARAMS_PATH));

        let secrets = Secrets {
            api_key: env::var("API_KEY").unwrap(),
            api_secret: env::var("API_SECRET").unwrap(),
        };

        SharedState {
            config,
            secrets,
            binance_ws_connected: false,
            binance_trades: AllocRingBuffer::new(1000),
            binance_bba: BestBidAsk {
                bid_price: 1.0,
                bid_qty: 1.0,
                ask_price: 1.0,
                ask_qty: 1.0,
            },
            binance_orderbook: OrderBookBinance::new(),
            binance_last_price: 0.0,

            bybit_ws_connected: false,
            bybit_klines: AllocRingBuffer::new(500),
            bybit_trades: AllocRingBuffer::new(1000),
            bybit_bba: BestBidAsk {
                bid_price: 1.0,
                bid_qty: 1.0,
                ask_price: 1.0,
                ask_qty: 1.0,
            },
        }
    }
}
