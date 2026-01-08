use crate::types::{Candlestick, Order, OrderSide};
use super::Strategy;

pub struct EmaCrossover {
    symbol: String,
    fast_alpha: f64,
    slow_alpha: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    in_position: bool,
}

impl EmaCrossover {
    pub fn new(symbol: String, fast: usize, slow: usize) -> Self {
        Self {
            symbol,
            fast_alpha: 2.0 / (fast as f64 + 1.0),
            slow_alpha: 2.0 / (slow as f64 + 1.0),
            fast_ema: None,
            slow_ema: None,
            in_position: false,
        }
    }

    fn update_ema(price: f64, prev: Option<f64>, alpha: f64) -> f64 {
        match prev {
            Some(p) => alpha * price + (1.0 - alpha) * p,
            None => price,
        }
    }
}

impl Strategy for EmaCrossover {
    fn on_bar(&mut self, bar: &Candlestick) -> Option<Order> {
        self.fast_ema = Some(Self::update_ema(bar.close, self.fast_ema, self.fast_alpha));
        self.slow_ema = Some(Self::update_ema(bar.close, self.slow_ema, self.slow_alpha));

        let fast = self.fast_ema.unwrap();
        let slow = self.slow_ema.unwrap();

        if fast > slow && !self.in_position {
            self.in_position = true;
            Some(Order {
                symbol: self.symbol.clone(),
                side: OrderSide::Buy,
                volume: 1.0,
            })
        } else if fast < slow && self.in_position {
            self.in_position = false;
            Some(Order {
                symbol: self.symbol.clone(),
                side: OrderSide::Sell,
                volume: 1.0,
            })
        } else {
            None
        }
    }
}
