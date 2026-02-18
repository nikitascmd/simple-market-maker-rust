use anyhow::Result;
use std::collections::HashMap;

use crate::shared_state::SharedState;

#[derive(Debug)]
pub struct PriceLevel {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug)]
pub struct OrderBookBybit {
    asks: Vec<PriceLevel>,
    bids: Vec<PriceLevel>,
}

impl OrderBookBybit {
    pub fn new() -> Self {
        Self {
            asks: Vec::new(),
            bids: Vec::new(),
        }
    }

    fn process_snapshot(&mut self, asks: Vec<PriceLevel>, bids: Vec<PriceLevel>) -> () {
        self.asks = asks;
        self.bids = bids;
        self.sort_book();
    }

    fn sort_book(&mut self) -> () {
        self.asks
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        self.asks.truncate(500);

        self.bids
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        self.bids.truncate(500);
    }

    fn update_book(
        &self,
        mut asks_or_bids: Vec<PriceLevel>,
        data: Vec<[String; 2]>,
    ) -> Result<Vec<PriceLevel>> {
        for [price, qty] in &data {
            let price_f64: f64 = price.parse()?;
            let qty_f64: f64 = qty.parse()?;

            asks_or_bids.retain(|level| level.price != price_f64);

            if qty_f64 > 0.0 {
                asks_or_bids.push(PriceLevel {
                    price: price_f64,
                    quantity: qty_f64,
                });
            }
        }

        return Ok(asks_or_bids);
    }

    // pub fn process_snapshot(&mut self, snapshot: HashMap<String, Value>)
}

pub struct BybitBBAHandler {
    shared_state: SharedState,
}

impl BybitBBAHandler {
    fn new(shared_state: SharedState) -> BybitBBAHandler {
        Self { shared_state }
    }
}
