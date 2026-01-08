use std::path::Path;

use advanced::broker::Broker;
use advanced::market_data::MarketData;
use advanced::strategy::Strategy;
use advanced::strategy::ema_strategy::EmaCrossover;
use advanced::types::{Candlestick, OrderSide};
use csv::Reader;

fn main() {
    let mut exness = Broker::new(String::from("Exness"));

    let account = exness.create_account("denis".to_string());
    account.deposit(1000.0);
    let path = Path::new("../data/apple_data.csv");

    let mut data_reader = Reader::from_path(path).unwrap();
    let mut bars = vec![];

    for bar in data_reader.deserialize() {
        let candlestick: Candlestick = bar.unwrap();
        bars.push(candlestick);
    }

    let mut market = MarketData::new(bars);

    let mut strategy = EmaCrossover::new(String::from("AAPL"), 21, 50);
    let mut order_id: u64 = 0;
    while let Some(bar) = market.next() {
        if let Some(order) = strategy.on_bar(&bar) {
            if let OrderSide::Buy = order.side {
                order_id = account.execute(order, bar.close);
            } else {
               println!("{}",account.close_position(order_id, bar.close).unwrap());
               println!();
            }

        }
    }
}
