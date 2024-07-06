use std::str::FromStr;

use near_sdk::{log, require};

use crate::{event::FtTransfer, *};

use self::ft_core::FungibleTokenCore;

pub const ZERO_TOKEN: NearToken = NearToken::from_near(0);
pub const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

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

    pub(crate) fn assert_one_yocto(&self) {
        if env::attached_deposit().ne(&ONE_YOCTO) {
            env::panic_str("You must attach exactly 1 yocto to call this method")
        }
    }

    pub(crate) fn internal_withdraw(&mut self, account_id: &AccountId, amount: NearToken) {
        let balance = self.internal_unwrap_balance_of(account_id);

        if let Some(new_balance) = balance.checked_sub(amount) {
            self.accounts.insert(&account_id, &new_balance);
        } else {
            env::panic_str("The account doesn't have enough balance");
        }
    }

    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: NearToken,
        memo: Option<String>,
    ) {
        require!(
            sender_id != receiver_id,
            "Sender and receiver should be different"
        );
        require!(
            amount.gt(&ZERO_TOKEN),
            "The amount should be a positive number"
        );

        self.internal_withdraw(sender_id, amount);
        self.internal_deposit(receiver_id, amount);

        FtTransfer {
            old_owner: sender_id,
            new_owner_id: receiver_id,
            amount: &amount,
            memo: memo.as_deref(),
        }
        .emit()
    }
}
