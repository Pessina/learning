use near_sdk::json_types::{U128, U64};
use near_sdk::{env, log, near, require, AccountId, NearToken, Promise};

use crate::Contract;
use crate::ContractExt;

pub const STORAGE_COST: NearToken = NearToken::from_millinear(1);

#[near(serializers = [json])]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near]
impl Contract {
    #[payable]
    pub fn donate(&mut self) -> String {
        let donor = env::predecessor_account_id();
        let donation_amount = env::attached_deposit();

        require!(
            donation_amount > STORAGE_COST,
            format!(
                "Attach at least {} yoctoNEAR to cover for the storage cost",
                STORAGE_COST
            )
        );

        let donated_so_far = self
            .donations
            .get(&donor)
            .unwrap_or(NearToken::from_near(0));

        let amount_to_transfer = if donated_so_far.is_zero() {
            donation_amount.saturating_sub(STORAGE_COST)
        } else {
            donation_amount
        };

        self.donations
            .insert(&donor, &donated_so_far.saturating_add(amount_to_transfer));

        log!(
            "Thank you {} for donating {}! You donated a total of {}",
            donor.clone(),
            donation_amount,
            donated_so_far
        );

        Promise::new(self.beneficiary.clone()).transfer(amount_to_transfer);

        donated_so_far.to_string()
    }

    pub fn get_donations(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<Donation> {
        let from = u64::from(from_index.unwrap_or(U64::from(0)));
        let limit = u64::from(limit.unwrap_or(U64::from(10)));

        self.donations
            .into_iter()
            .skip(from as usize)
            .take(limit as usize)
            .map(|(account_id, total_amount)| Donation {
                account_id,
                total_amount: U128::from(total_amount.as_yoctonear()),
            })
            .collect()
    }

    pub fn get_donation_for_account(&self, account_id: AccountId) -> Donation {
        let amount = self
            .donations
            .get(&account_id)
            .unwrap_or(NearToken::from_near(0))
            .as_yoctonear();

        Donation {
            account_id,
            total_amount: U128::from(amount),
        }
    }

    pub fn number_of_donors(&self) -> U64 {
        U64::from(self.donations.len())
    }
}
