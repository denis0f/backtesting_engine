use std::{collections::HashMap};

use crate::broker::accounts::Account;


pub mod accounts;


pub struct Broker{
    name: String,
    pub accounts: HashMap<u64, Account>,
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
    fn update_id(&mut self){
        self.id+=1;
    }
    fn update_accounts(&mut self, name: String, account: Account){
        self.accounts.insert(account.id, account);

    }
    
    pub fn create_account(&mut self, name: String) -> &mut Account{
        self.id+=1;
        let id = self.id;

        self.accounts.entry(id).or_insert_with(|| Account::new(id, name))
    }

}