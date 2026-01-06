use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketBar{
    pub timestamp: String,
    pub close: f64
}

#[derive(Debug, Clone)]
pub enum OrderSide{
    Buy, 
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order{
    pub side: OrderSide,
    pub quantity: f64,
}

#[derive(Debug, Clone)]
pub struct Fill{
    pub order: Order, 
    pub price: f64
}