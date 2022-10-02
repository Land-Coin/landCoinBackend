use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet},
    near_bindgen,
};

use property::{Category, Property, Purpose};
use transaction::Transaction;

pub mod property;
pub mod transaction;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LandCoin {
    pub properties: UnorderedMap<String, Property>,
    pub transactions: UnorderedMap<String, Transaction>,
    pub categories: UnorderedSet<Category>,
    pub purposes: UnorderedSet<Purpose>,
}

#[near_bindgen]
impl LandCoin {
    #[init]
    pub fn new() -> Self {
        Self {
            properties: UnorderedMap::new(b"a"),
            transactions: UnorderedMap::new(b"b"),
            categories: UnorderedSet::new(b"c"),
            purposes: UnorderedSet::new(b"e"),
        }
    }
}

#[cfg(test)]
mod tests {

    // use super::*;
    use crate::property::{Coords, Dispute};
    use crate::*;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::test_env::{alice, bob, carol};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, ONE_NEAR};

    pub fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .current_account_id(alice())
            .signer_account_id(bob())
            .predecessor_account_id(bob())
            .is_view(is_view)
            .attached_deposit(0)
            .account_balance(0)
            .build()
    }

    fn prop_id() -> String {
        "property_1".to_string()
    }

    fn load_price(price: &str) -> String {
        price.to_string()
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn landcoin_test() {
        let _context = get_context(false);
        testing_env!(_context);
        let mut landcoin = LandCoin::new();

        landcoin.add_category("Land".to_string(), "Acres".to_string());
        landcoin.add_category("Building".to_string(), "Square metres".to_string());

        assert_eq!(landcoin.categories.len(), 2 , "Category not added");

        landcoin.add_purpose("Sell".to_string());
        landcoin.add_purpose("Rent".to_string());
        landcoin.add_purpose("Personal".to_string());

        assert_eq!(landcoin.purposes.len(), 3, "Purposes not added");
        let purpose = Purpose {
            name: "Sell".to_string(),
        };
        let mut owners = UnorderedSet::new(b"s");
        owners.insert(&"dalmasonto.testnet".to_string());
        owners.insert(&"timo.testnet".to_string());

        let mut owners_update = UnorderedSet::new(b"z");
        owners_update.insert(&"dalmasonto.testnet".to_string());

        let loc = Coords {
            lat: "1.2232".to_string(),
            lon: "36.1234".to_string(),
        };

        landcoin.add_property(
            prop_id(),
            "Land".to_string(),
            purpose.clone(),
            "2000".to_string(),
            "LAND 7668767".to_string(),
            owners,
            load_price("KES 30, 000"),
            loc,
        );

        assert_eq!(landcoin.properties.len(), 1, "Property not added");

        landcoin.update_property(prop_id(), true, true);

        landcoin.edit_property(prop_id(), purpose, owners_update, load_price("KES 40, 000"));

        let dispute = Dispute {
            raiser: alice(),
            message: "This land is ours".to_string(),
        };

        landcoin.add_dispute(prop_id(), dispute);

        let prop = landcoin.pub_get_property(prop_id());

        match prop {
            Some(k) => {
                assert_eq!(k.has_caveat, true, "Property has caveat not updated");
                assert_eq!(k.has_dispute, true, "Property has dispute not updated");
                assert_eq!(k.owners.len(), 1, "Property owners not updated");
                assert_eq!(
                    k.price,
                    load_price("KES 40, 000"),
                    "Property price not updated"
                );
                assert_eq!(k.disputes.len(), 1, "Property disputes not updated");
            }
            None => {}
        }

        landcoin.add_transaction(
            prop_id(),
            load_price("transaction1"),
            load_price("dalmasonto.testnet"),
            load_price("timo.testnet"),
            U128(3142343243243),
            load_price("Buy"),
        );

        assert_eq!(landcoin.transactions.len(), 1, "Transaction not done");
    }
}
