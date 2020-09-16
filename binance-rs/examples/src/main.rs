extern crate binance;

use binance::api::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::userstream::*;
use binance::websockets::*;
use binance::model::{AccountUpdateEvent, KlineEvent, OrderTradeEvent,
                     TradesEvent, DayTickerEvent, OrderBook, DepthOrderBookEvent};

fn main() {
    general();
    account();
    market_data();
    user_stream();
    user_stream_websocket();
    market_websocket();
    kline_websocket();
    all_trades_websocket();
}

fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
}

fn account() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_balance("KNC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }
}

fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);

    if let Ok(answer) = user_stream.start() {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(&listen_key) {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match user_stream.close(&listen_key) {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

fn user_stream_websocket() {
    struct WebSocketHandler;

    impl UserStreamEventHandler for WebSocketHandler {
        fn account_update_handler(&self, event: &AccountUpdateEvent) {
            for balance in &event.balance {
                println!(
                    "Asset: {}, free: {}, locked: {}",
                    balance.asset, balance.free, balance.locked
                );
            }
        }

        fn order_trade_handler(&self, event: &OrderTradeEvent) {
            println!(
                "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                event.symbol, event.side, event.price, event.execution_type
            );
        }
    }

    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets = WebSockets::new();
        web_socket.add_user_stream_handler(WebSocketHandler);
        web_socket.connect(&listen_key).unwrap(); // check error
        web_socket.event_loop();
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

fn market_websocket() {
    struct WebSocketHandler;

    impl MarketEventHandler for WebSocketHandler {
        fn aggregated_trades_handler(&self, event: &TradesEvent) {
            println!(
                "Symbol: {}, price: {}, qty: {}",
                event.symbol, event.price, event.qty
            );
        }

        fn depth_orderbook_handler(&self, event: &DepthOrderBookEvent) {
            println!(
                "Symbol: {}, Bids: {:?}, Ask: {:?}",
                event.symbol, event.bids, event.asks
            );
        }

        fn partial_orderbook_handler(&self, order_book: &OrderBook) {
            println!(
                "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                order_book.last_update_id, order_book.bids, order_book.asks
            );
        }
    }

    let agg_trade: String = format!("{}@aggTrade", "ethbtc");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_market_handler(WebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}

fn all_trades_websocket() {
    struct WebSocketHandler;

    impl DayTickerEventHandler for WebSocketHandler {
        fn day_ticker_handler(&self, events: &[DayTickerEvent]) {
            for event in events {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    event.symbol, event.best_bid, event.best_bid_qty
                );
            }
        }
    }

    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_day_ticker_handler(WebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}

fn kline_websocket() {
    struct WebSocketHandler;

    impl KlineEventHandler for WebSocketHandler {
        fn kline_handler(&self, event: &KlineEvent) {
            println!(
                "Symbol: {}, high: {}, low: {}",
                event.kline.symbol, event.kline.low, event.kline.high
            );
        }
    }

    let kline: String = format!("{}", "ethbtc@kline_1m");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_kline_handler(WebSocketHandler);
    web_socket.connect(&kline).unwrap(); // check error
    web_socket.event_loop();
}
