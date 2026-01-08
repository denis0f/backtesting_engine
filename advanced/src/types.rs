#[derive(Debug)]
pub enum OrderSide{
    Buy,
    Sell,
    Hold
}

#[derive(Debug)]
pub struct Order{
    pub id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub volume: f64,
}

#[derive(Debug)]
pub struct Position{
    pub id: u64,
    pub symbol: String,
    pub volume: f64,
    pub type_: OrderSide
}