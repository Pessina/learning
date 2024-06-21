use near_sdk::json_types::U64;
use near_sdk::store::Vector;
use near_sdk::{env, near, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct PostMessage {
    premium: bool,
    sender: AccountId,
    text: String,
}

#[near(contract_state)]
pub struct Contract {
    messages: Vector<PostMessage>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near]
impl Contract {
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }

    #[payable]
    pub fn add_message(&mut self, text: String) -> PostMessage {
        let post_message = PostMessage {
            premium: env::attached_deposit() >= POINT_ONE,
            sender: env::predecessor_account_id(),
            text: text.clone(),
        };

        self.messages.push(post_message.clone());
        post_message.clone()
    }

    pub fn get_messages(&self, from: Option<U64>, limit: Option<U64>) -> Vec<&PostMessage> {
        let from = u64::from(from.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}
