use std::{collections::HashMap};

use crate::broker::accounts::Account;


pub mod accounts;


struct Broker{
    name: String,
    accounts: HashMap<u64, Account>,
    id: u64
}

impl Broker{
    pub fn new(name: String) -> Self{
        Self {
            name,
            accounts: HashMap::new(),
            id: 0
        }
    }
    
    pub fn create_account(&mut self, name: String) -> Option<&Account>{
        self.id+=1;
        let account = Account::new(self.id, name);
        self.accounts.insert(account.id, account);

        self.accounts.get(&self.id)
    }
}