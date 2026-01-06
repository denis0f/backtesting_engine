use simple::broker::Broker;
use simple::data_feed_tick::MarketData;
use simple::strategy::sma_strategy::SmaStrategy;
use simple::types::MarketBar;

use csv::Reader;

fn main() {
    let mut bars = vec![];
    //reading the data from the csv
    let path = std::path::Path::new("../data/apple_data_4h.csv");

    let mut reader = Reader::from_path(path).unwrap();


    for record in reader.records() {
        let myrecord = record.unwrap();

        if let Some(date) = myrecord.get(0) {
            if let Some(close) = myrecord.get(2) {
                bars.push(MarketBar {
                    timestamp: date.to_string(),
                    close: close.parse::<f64>().unwrap(),
                })
            }
        }
    }

    let mut market = MarketData::new(bars);
    let mut strategy = SmaStrategy::new(3, 2);
    let mut broker = Broker::new(10_000.0);

    while let Some(bar) = market.next() {
        if let Some(order) = strategy.on_bar(&bar) {
            let fill = broker.execute(order, bar.close);
            println!("Filled: {:?}", fill);
        }
    }

    println!();
    println!("Final cash is: {}", broker.balance);
    println!("Final position is: {}", broker.position);
}
