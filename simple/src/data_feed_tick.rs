use crate::types::MarketBar;

pub struct MarketData{
    bars: Vec<MarketBar>,
    index: usize
}

impl MarketData{
    pub fn new(bars: Vec<MarketBar>) -> Self{
        Self {
            bars, 
            index: 0
        }
    }

    pub fn next(&mut self)-> Option<MarketBar>{
        if self.index >= self.bars.len(){
            return None
        } else {
            let bar = self.bars[self.index].clone();
            self.index += 1;
            Some(bar)
        }
    }
}