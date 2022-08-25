use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    symbol: String,
    price_change: String,
    price_change_percent: String,
    weighted_avg_price: String,
    prev_close_price: String,
    last_price: String,
    last_qty: String,
    bid_price: String,
    bid_qty: String,
    ask_price: String,
    ask_qty: String,
    open_price: String,
    high_price: String,
    low_price: String,
    volume: String,
    quote_volume: String,
    open_time: u64,
    close_time: u64,
    first_id: u128,
    last_id: u128,
    count: u32,
}

impl Ticker {
    fn to_instrument(&self) -> Instrument {
        Instrument {
            ex_name: "Binance".to_string(),
            symbol: self.symbol.clone(),
            price: self.last_price.parse::<f32>().unwrap_or(0f32),
            price_24h_change_percent: self.price_change_percent.parse::<f32>().unwrap_or(0f32),
            volume: self.quote_volume.parse::<f64>().unwrap_or(0f64),
        }
    }
}

pub struct Binance;

impl SymbolFormatter for Binance {
    fn format_symbol(&self, base: String) -> String {
        format!("{}{}", base, QUOTE.to_string())
    }
}

impl Exchange for Binance {
    fn get_instrument(&self, symbol: String) -> Result<Instrument, Box<dyn std::error::Error>> {
        let ticker = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbol", symbol)])
            .send()?
            .json::<Ticker>()?;

        Ok(ticker.to_instrument())
    }

    fn get_instruments(
        &self,
        symbols: Vec<String>,
    ) -> Result<Vec<Instrument>, Box<dyn std::error::Error>> {
        let tickers = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbols", format!("{:?}", symbols).replace(" ", ""))])
            .send()?
            .json::<Vec<Ticker>>()?;

        Ok(tickers.iter().map(|t| t.to_instrument()).collect())
    }
}
