pub mod ema_strategy;
use crate::types::{Candlestick, Order};

pub trait Strategy {
    fn on_bar(&mut self, bar: &Candlestick) -> Option<Order>;
}
