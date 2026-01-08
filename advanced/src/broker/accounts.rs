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

    pub fn execute(&mut self, order: Order, price: f64) -> Option<&Position> {
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

        self.positions.get(&self.position_id_track)
    }
    pub fn close_position(&mut self, positions_id: u64, price: f64) -> Result<String, Box<dyn std::error::Error>> {
        let position = self.positions.get(&positions_id);
        let removed_position = match position {
            Some(positions) => {
                let proceeds = price * positions.volume;
                self.balance += proceeds;

                self.positions.remove(&positions_id)
            }
            _ => None
        };

        match removed_position{
            Some(x) => {
    Ok(format!("Closed Position; Symbol: {}, Id: {}, Volume: {}", &x.symbol, &x.id, &x.volume))
            }
            None => Ok(format!("Failed to remove the position confirm the id the try again."))
        }

    }
}
