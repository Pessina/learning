use near_sdk::{env, log, near, serde_json::json, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_ARGS, NO_DEPOSIT, XCC_GAS};

#[near]
impl Contract {
    pub fn batch_actions(&mut self) -> Promise {
        let hi = json!({ "greeting": "hi"  }).to_string().into_bytes();
        let bye = json!({"greeting": "bye"}).to_string().into_bytes();

        Promise::new(self.hello_account.clone())
            .function_call("set_greeting".to_owned(), hi, NO_DEPOSIT, XCC_GAS)
            .function_call("get_greeting".to_owned(), NO_ARGS, NO_DEPOSIT, XCC_GAS)
            .function_call("set_greeting".to_owned(), bye, NO_DEPOSIT, XCC_GAS)
            .function_call("get_greeting".to_owned(), NO_ARGS, NO_DEPOSIT, XCC_GAS)
            .then(Self::ext(env::current_account_id()).batch_actions_callback())
    }

    #[private]
    pub fn batch_actions_callback(
        &self,
        #[callback_result] last_result: Result<String, PromiseError>,
    ) -> String {
        if let Ok(result) = last_result {
            log!(format!("The last result is {result}"));
            result
        } else {
            log!("The batch call failed and all calls got reverted");
            "".to_string()
        }
    }
}
