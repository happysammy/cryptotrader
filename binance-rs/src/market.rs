use client::*;
use errors::*;
use model::*;
use serde_json::from_str;
use std::collections::BTreeMap;
use util::*;

#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl Market {
    pub fn get_klines<S>(&self, symbol: S, inverval: S) -> Result<(Vec<CandleStick>)>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), inverval.into());

        let request = build_request(&parameters);
        let data = self.client.get("/api/v1/klines", &request)?;
        let order_book_result: Vec<(
            u64,
            String,
            String,
            String,
            String,
            String,
            u64,
            String,
            u64,
            String,
            String,
            String,
        )> = from_str(data.as_str()).unwrap();

        Ok(order_book_result
            .iter()
            .map(|r| CandleStick {
                open_time: r.0,
                open_price: r.1.parse::<f64>().unwrap(),
                high_price: r.2.parse::<f64>().unwrap(),
                low_price: r.3.parse::<f64>().unwrap(),
                close_price: r.4.parse::<f64>().unwrap(),
                volume: r.5.parse::<f64>().unwrap(),
                quote_asset_volume: r.7.parse::<f64>().unwrap(),
                trades: r.8,
                taker_buy_base_asset_volume: r.9.parse::<f64>().unwrap(),
                taker_buy_quote_asset_volume: r.10.parse::<f64>().unwrap(),
            })
            .collect())
    }

    // Order book (Default 100; max 100)
    pub fn get_depth<S>(&self, symbol: S) -> Result<(OrderBook)>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_request(&parameters);
        let data = self.client.get("/api/v1/depth", &request)?;
        let order_book: OrderBook = from_str(data.as_str()).expect("thing");

        Ok(order_book)
    }

    // Latest price for ALL symbols.
    pub fn get_all_prices(&self) -> Result<(Prices)> {
        let data = self.client.get("/api/v1/ticker/allPrices", "")?;

        let prices: Prices = from_str(data.as_str()).unwrap();

        Ok(prices)
    }

    // Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<(f64)>
    where
        S: Into<String>,
    {
        match self.get_all_prices() {
            Ok(answer) => match answer {
                Prices::AllPrices(prices) => {
                    let cmp_symbol = symbol.into();
                    for par in prices {
                        if par.symbol == cmp_symbol {
                            return Ok(par.price);
                        }
                    }
                    bail!("Symbol not found");
                }
            },
            Err(e) => Err(e),
        }
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<(BookTickers)> {
        let data = self.client.get("/api/v1/ticker/allBookTickers", "")?;

        let book_tickers: BookTickers = from_str(data.as_str()).unwrap();

        Ok(book_tickers)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<(Tickers)>
    where
        S: Into<String>,
    {
        match self.get_all_book_tickers() {
            Ok(answer) => match answer {
                BookTickers::AllBookTickers(book_tickers) => {
                    let cmp_symbol = symbol.into();
                    for obj in book_tickers {
                        if obj.symbol == cmp_symbol {
                            let ticker: Tickers = obj;
                            return Ok(ticker);
                        }
                    }
                    bail!("Symbol not found");
                }
            },
            Err(e) => Err(e),
        }
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<(PriceStats)>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/ticker/24hr", &request)?;

        let stats: PriceStats = from_str(data.as_str()).unwrap();

        Ok(stats)
    }
}
