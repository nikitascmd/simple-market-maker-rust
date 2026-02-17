use std::collections::HashMap;

pub struct PriceLevel {
    pub price: f64,
    pub quantity: f64,
}

pub struct OrderBookBinance {
    asks: Vec<PriceLevel>,
    bids: Vec<PriceLevel>,
}

impl OrderBookBinance {
    // pub fn process_snapshot(&mut self, snapshot: HashMap<String, Value>)
}
