use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    Timestamp,
};

use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TType {
    pub name: String,
}

pub enum Ttype {
    String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub t_type: String,
    pub property: String,
    pub created_on: Timestamp,
}

impl Transaction {
    pub fn new(
        id: String,
        sender: String,
        receiver: String,
        amount: U128,
        t_type: String,
        property: String,
    ) -> Self {
        Self {
            id,
            sender,
            receiver,
            amount: u128::from(amount),
            t_type,
            property,
            created_on: env::block_timestamp(),
        }
    }
}

impl LandCoin {
    pub fn add_transaction(
        &mut self,
        property: String,
        id: String,
        sender: String,
        receiver: String,
        amount: U128,
        t_type: String,
    ) -> String {
        let prop = self.get_property(property.clone());
        if prop.as_ref().is_some() {
            let trans = Transaction::new(id.clone(), sender, receiver, amount, t_type, property);
            self.transactions.insert(&id.clone(), &trans);
            return "success".to_string();
        }
        return "failed".to_string();
    }
}
