use near_sdk::env;
use near_sdk::near;

use crate::Contract;
use crate::ContractExt;

#[near]
impl Contract {
    #[private]
    pub fn update_stored_contract(&mut self) {
        self.code.set(env::input());
    }

    pub fn get_code(&self) -> &Vec<u8> {
        self.code.get().as_ref().unwrap()
    }
}
