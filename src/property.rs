use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Category {
    pub name: String,
    pub units: String,

}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Purpose {
    pub name: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Coords {
    pub lat: String,
    pub lon: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Dispute {
    pub raiser: AccountId,
    pub message: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
// #[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Property {
    pub id: String,
    pub category: String,
    pub purpose: Purpose,
    pub size: String,
    pub title_no: String,
    pub owners: Vec<String>,
    pub price: String,
    pub location: Coords,
    pub has_caveat: bool,
    pub has_dispute: bool,
    pub disputes: Vec<Dispute>,
}

impl Property {
    pub fn new(
        id: String,
        category: String,
        purpose: Purpose,
        size: String,
        title_no: String,
        owners: Vec<String>,
        price: String,
        location: Coords,
    ) -> Self {
        Self {
            id,
            category,
            purpose,
            size,
            title_no,
            owners,
            price,
            location,
            has_caveat: false,
            has_dispute: false,
            disputes: Vec::new(),
        }
    }
    pub fn add_dispute(&mut self, dispute: Dispute) {
        self.disputes.push(dispute);
    }

    pub fn edit_bools(&mut self, has_cav: bool, has_dis: bool) {
        self.has_caveat = has_cav;
        self.has_dispute = has_dis;
    }

    pub fn edit(&mut self, purpose: Purpose, owners: Vec<String>, price: String){
        self.purpose = purpose;
        self.owners = owners;
        self.price = price
    }
}


#[near_bindgen]
impl LandCoin {
    pub fn add_property(
        &mut self,
        id: String,
        category: String,
        purpose: Purpose,
        size: String,
        title_no: String,
        owners: Vec<String>,
        price: String,
        location: Coords,
    ) -> String {
        let property = Property::new(
            id.clone(),
            category,
            purpose,
            size,
            title_no,
            owners,
            price,
            location,
        );
        self.properties.insert(&id.clone(), &property);
        return "success".to_string();
    }

    pub fn get_property(&mut self, id: String) -> Option<Property> {
        self.properties.get(&id)
    }

    pub fn search_property(&mut self, query: String) -> Option<Property> {
        let prop = self.properties.values().into_iter().find(|k| k.id == query || k.title_no == query);
        return prop;
    }

    pub fn pub_get_property(&self, id: String) -> Option<Property> {
        self.properties.get(&id)
    }

    pub fn add_dispute(&mut self, prop_id: String, dispute: Dispute) -> String {
        let mut prop = self.get_property(prop_id.clone());
        if prop.as_ref().is_some() {
            prop.as_mut().unwrap().add_dispute(dispute);
            self.properties.insert(&prop_id.clone(), &prop.unwrap());
            return "success".to_string();
        }
        return "failed".to_string();
    }

    pub fn update_property(
        &mut self,
        prop_id: String,
        has_caveat: bool,
        has_dispute: bool,
    ) -> String {
        let mut prop = self.get_property(prop_id.clone());
        if prop.as_ref().is_some() {
            prop.as_mut().unwrap().edit_bools(has_caveat, has_dispute);
            self.properties.insert(&prop_id.clone(), &prop.unwrap());
            return "success".to_string();
        }
        return "failed".to_string();
    }

    pub fn add_category(&mut self, category: String, units: String) -> String {
        let cat = Category { name: category, units };
        self.categories.insert(&cat);
        return "success".to_string();
    }

    pub fn add_purpose(&mut self, purpose: String) -> String {
        let pup = Purpose { name: purpose };
        self.purposes.insert(&pup);
        return "success".to_string();
    }

    pub fn edit_property(&mut self, prop_id: String, purpose: Purpose, owners: Vec<String>, price: String, )-> String{
        let mut prop = self.get_property(prop_id.clone());
        if prop.as_ref().is_some() {
            prop.as_mut().unwrap().edit(purpose, owners, price);
            self.properties.insert(&prop_id.clone(), &prop.unwrap());
            return "success".to_string();
        }
        return "failed".to_string();
    }
}
