use near_sdk::collections::UnorderedMap;
use near_sdk::{near, AccountId, NearToken};

mod donation;

#[near(contract_state)]
pub struct Contract {
    donations: UnorderedMap<AccountId, NearToken>,
    beneficiary: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            donations: UnorderedMap::new(b"d"),
            beneficiary: "mighty-baby.testnet".parse::<AccountId>().unwrap(),
        }
    }
}

#[near]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        Self {
            donations: UnorderedMap::new(b"d"),
            beneficiary: "mighty-baby.testnet".parse::<AccountId>().unwrap(),
        }
    }

    pub fn get_beneficiary(&self) -> &AccountId {
        &self.beneficiary
    }

    #[private]
    pub fn change_beneficiary(&mut self, new_beneficiary: AccountId) {
        self.beneficiary = new_beneficiary;
    }
}
