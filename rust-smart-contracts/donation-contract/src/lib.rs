use near_sdk::collections::UnorderedMap;
use near_sdk::{near, AccountId, NearToken, PanicOnDefault};

mod donation;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub beneficiary: AccountId,
    pub donations: UnorderedMap<AccountId, NearToken>,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(beneficiary: AccountId) -> Self {
        Self {
            beneficiary,
            donations: UnorderedMap::new(b"d"),
        }
    }

    pub fn get_beneficiary(&self) -> &AccountId {
        &self.beneficiary
    }

    #[private]
    pub fn change_beneficiary(&mut self, new_beneficiary: AccountId) {
        self.beneficiary = new_beneficiary
    }
}
