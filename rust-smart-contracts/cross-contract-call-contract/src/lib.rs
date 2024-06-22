use near_sdk::{env, log, near, AccountId, Gas, PanicOnDefault, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    hello_account: AccountId,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(hello_account: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { hello_account }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(hello_account: AccountId) -> Self {
        Self { hello_account }
    }

    pub fn query_greeting(&self) -> Promise {
        let promise = hello_near::ext(self.hello_account.clone())
            .with_static_gas(Gas::from_tgas(5))
            .get_greeting();

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(),
        )
    }

    #[private]
    pub fn query_greeting_callback(
        &self,
        #[callback_result] call_result: Result<String, PromiseError>,
    ) -> String {
        if call_result.is_err() {
            log!("There was an error contacting zHello NEAR");
            return "".to_string();
        }

        let greeting: String = call_result.unwrap();
        greeting
    }

    pub fn change_greeting(&self, new_greeting: String) -> Promise {
        let promise = hello_near::ext(self.hello_account.clone())
            .with_static_gas(Gas::from_tgas(5))
            .set_greeting(new_greeting);

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .change_greeting_callback(),
        )
    }

    pub fn change_greeting_callback(
        &self,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            env::log_str("set_greeting failed...");
            return false;
        }

        env::log_str("set_greeting was successful!");
        true
    }
}
