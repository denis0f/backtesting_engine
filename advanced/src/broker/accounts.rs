use crate::types::{Order, Position};
use std::{collections::HashMap, fmt::format};

#[derive(Debug)]
pub struct Account {
    pub id: u64,
    pub name: String,
    balance: f64,
    position_id_track: u64,
    positions: HashMap<u64, Position>,
}

impl Account {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            balance: 0.0,
            positions: HashMap::new(),
            position_id_track: 0,
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_positions(&self) -> &HashMap<u64, Position> {
        &self.positions
    }

    pub fn execute(&mut self, order: Order, price: f64) -> u64 {
        let cost = order.volume * price;
        self.balance -= cost;
        self.position_id_track += 1;
        let position = Position {
            id: self.position_id_track,
            symbol: order.symbol,
            volume: order.volume,
            type_: order.side,
        };
        self.positions.insert(self.position_id_track, position);
        let pst = self.positions.get(&self.position_id_track).unwrap();

        println!("Executed Position: id: {}, symbol: {}, volume: {}, type: {:?}, price: {}", pst.id, pst.symbol, pst.volume, pst.type_, price);
        println!("Account balance is {}", self.balance);
        println!();

        self.position_id_track
    }
    pub fn close_position(
        &mut self,
        position_id: u64,
        price: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(position) = self.positions.remove(&position_id) {
            let proceeds = price * position.volume;
            self.balance += proceeds;

            Ok(format!(
                "Closed Position; Symbol: {}, Id: {}, Volume: {}, price {}, current balance is {}",
                position.symbol, position.id, position.volume, price, self.balance
            ))
        } else {
            Ok("Failed to remove the position. Confirm the ID and try again.".to_string())
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited amount: {}, New balance is: {}",
                amount, self.balance
            );
        } else {
            println!("Amount should be greater than 0.");
        }
    }
}
