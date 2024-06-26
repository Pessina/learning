mod migrate;

use near_sdk::{
    env,
    json_types::{U128, U64},
    near,
    store::Vector,
    AccountId, NearToken,
};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[near(serializers = [borsh, json])]
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(contract_state)]
pub struct GuestBook {
    messages: Vector<PostedMessage>,
}

impl Default for GuestBook {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near]
impl GuestBook {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let sender = env::predecessor_account_id();
        let premium = payment >= POINT_ONE;

        let message = PostedMessage {
            payment,
            premium,
            sender,
            text,
        };

        self.messages.push(message)
    }

    pub fn get_messages(
        &self,
        from_index: Option<U128>,
        limit: Option<U64>,
    ) -> Vec<&PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128::from(0)));

        self.messages
            .into_iter()
            .skip(from as usize)
            .take(u64::from(limit.unwrap_or(U64::from(10))) as usize)
            .collect()
    }
}
