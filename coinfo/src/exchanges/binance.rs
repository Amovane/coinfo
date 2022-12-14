use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    symbol: String,
    price_change_percent: String,
    last_price: String,
    quote_volume: String,
}

impl Ticker {
    fn converted(&self) -> super::Ticker {
        super::Ticker {
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
        format!("{}{}", base, QUOTE.to_string()).to_uppercase()
    }
}

impl Exchange for Binance {
    fn get_ticker(&self, symbol: String) -> Result<super::Ticker, Box<dyn std::error::Error>> {
        let ticker = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbol", symbol)])
            .send()?
            .json::<Ticker>()?;

        Ok(ticker.converted())
    }

    fn get_tickers(
        &self,
        symbols: Vec<String>,
    ) -> Result<Vec<super::Ticker>, Box<dyn std::error::Error>> {
        let tickers = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbols", format!("{:?}", symbols).replace(" ", ""))])
            .send()?
            .json::<Vec<Ticker>>()?;

        Ok(tickers.iter().map(|t| t.converted()).collect())
    }
}
