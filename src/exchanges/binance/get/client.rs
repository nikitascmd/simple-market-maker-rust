use anyhow::Result;
use binance_sdk::{
    config::ConfigurationRestApi,
    spot::rest_api::{DepthParams, KlinesParams, RestApi},
};

use crate::{
    exchanges::binance::websockets::handlers::orderbook::PriceLevel,
    shared_state::SharedState,
};

pub struct BinancePublicGet {
    shared_state: SharedState,
    client: RestApi,
}

impl BinancePublicGet {
    pub fn new(shared_state: SharedState) -> Result<BinancePublicGet> {
        let mut rest_config = ConfigurationRestApi::builder().build()?;

        rest_config.base_path = Some("https://api.binance.com".to_string());

        let client = RestApi::new(rest_config);
        Ok(BinancePublicGet {
            shared_state,
            client,
        })
    }

    pub async fn orderbook(&self, limit: i32) -> Result<(Vec<PriceLevel>, Vec<PriceLevel>)> {
        let params = DepthParams::builder(self.shared_state.config.binance_symbol.clone())
            .limit(limit)
            .build()?;

        let response = self.client.depth(params).await?;
        let data = response.data().await?;

        let bids = self.parse_levels(data.bids);
        let asks = self.parse_levels(data.asks);

        Ok((bids, asks))
    }

    fn parse_levels(&self, raw: Option<Vec<Vec<String>>>) -> Vec<PriceLevel> {
        raw.unwrap_or_default()
            .iter()
            .filter_map(|level| {
                Some(PriceLevel {
                    price: level[0].parse().ok()?,
                    quantity: level[1].parse().ok()?,
                })
            })
            .collect()
    }
}
