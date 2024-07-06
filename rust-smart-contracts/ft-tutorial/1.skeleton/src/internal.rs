use std::str::FromStr;

use near_sdk::require;

use crate::*;

pub const ZERO_TOKEN: NearToken = NearToken::from_near(0);

impl Contract {
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: NearToken) {
        let balance = self.internal_unwrap_balance_of(account_id);

        match balance.checked_add(amount) {
            Some(new_balance) => {
                self.accounts.insert(&account_id, &new_balance);
            }
            None => env::panic_str("Balance overflow"),
        }
    }

    pub(crate) fn measure_bytes_for_longest_account_id(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::from_str(&"a".repeat(64)).unwrap();
        self.accounts.insert(&tmp_account_id, &ZERO_TOKEN);
        self.bytes_for_longest_account_id = env::storage_usage() - initial_storage_usage;
        self.accounts.remove(&tmp_account_id);
    }

    pub(crate) fn internal_register_account(&mut self, account_id: &AccountId) {
        if self.accounts.insert(&account_id, &ZERO_TOKEN).is_some() {
            env::panic_str("The account is already registered")
        }
    }

    pub(crate) fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> NearToken {
        match self.accounts.get(account_id) {
            Some(balance) => balance,
            None => {
                env::panic_str(format!("The account {} is not registered", account_id).as_str())
            }
        }
    }
}
