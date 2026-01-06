use crate::types::{Fill, Order, OrderSide};

pub struct Broker{
    pub balance: f64,
    pub position: f64,
}

impl Broker {
    pub fn new(amount: f64) -> Self{
        Self { balance: amount, position: 0.0 }
    }

    pub fn execute(&mut self, order: Order, price: f64) -> Fill{
        match order.side {
            OrderSide::Buy => {
                let cost = order.quantity * price;
                self.balance -= cost;
                self.position += order.quantity
            }
            OrderSide::Sell => {
                let cost = order.quantity * price;
                self.balance += cost;
                self.position -= order.quantity;
            }
        }

        Fill{
            order,
            price
        }
    }
}
