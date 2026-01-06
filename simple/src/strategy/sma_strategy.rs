use crate::{ types::MarketBar, types::{OrderSide, Order}};

pub struct SmaStrategy{
    long_period: usize,
    short_period: usize,
    prices: Vec<f64>,
    in_position: bool
}

impl SmaStrategy{
    pub fn new(long_period: usize, short_period: usize) -> Self{
        Self { long_period, short_period, prices: Vec::new(), in_position: false }
    }

    fn sma(prices: &[f64]) -> f64{
        prices.iter().sum::<f64>() / prices.len() as f64
    }

    pub fn on_bar(&mut self, bar: &MarketBar)-> Option<Order>{
        self.prices.push(bar.close);

        if self.prices.len() < self.long_period {
            return None
        }

        let short_sma = Self::sma(&self.prices[self.prices.len() - self.short_period..]);
        let long_sma = Self::sma(&self.prices[self.prices.len() - self.long_period..]);

        if short_sma > long_sma && !self.in_position{
            self.in_position = true;
            Some(Order{
                side: OrderSide::Buy,
                quantity: 1.0
            })
        } else if short_sma < long_sma && self.in_position{
            self.in_position = false;
            Some(Order{
                side: OrderSide::Sell,
                quantity: 1.0,
            })
        } else {
            None
        }
    }
            
}