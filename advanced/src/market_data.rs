use crate::types::Candlestick;


pub struct MarketData{
    bars: Vec<Candlestick>,
    index: usize,
}

impl MarketData{
    pub fn new(bars: Vec<Candlestick>) -> Self{
        Self{
            bars,
            index: 0
        }
    }

    pub fn next(&mut self) -> Option<Candlestick>{
        if self.index >= self.bars.len(){
            None
        } else {
            let bar = self.bars[self.index].clone();
            self.index += 1;
            Some(bar)
        }
    }
}