use near_sdk::require;

use crate::*;

pub const ZERO_TOKEN: NearToken = NearToken::from_near(0);

impl Contract {
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: NearToken) {
        let balance = self.accounts.get(&account_id).unwrap_or(ZERO_TOKEN);

        match balance.checked_add(amount) {
            Some(new_balance) => {
                self.accounts.insert(&account_id, &new_balance);
            }
            None => env::panic_str("Balance overflow"),
        }
    }
}
