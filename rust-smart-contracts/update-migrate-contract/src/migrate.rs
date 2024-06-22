use crate::*;

#[near(serializers = [borsh])]
pub struct OldPostMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}
#[near(serializers = [borsh])]
pub struct OldState {
    messages: Vector<OldPostMessage>,
    payments: Vector<NearToken>,
}

#[near]
impl GuestBook {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldState = env::state_read().expect("Failed to read state");

        let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

        for (idx, posted) in old_state.messages.iter().enumerate() {
            let default = NearToken::from_near(0);
            let payment = old_state.payments.get(idx as u32).unwrap_or(&default);

            new_messages.push(PostedMessage {
                payment: *payment,
                premium: posted.premium,
                sender: posted.sender.clone(),
                text: posted.text.clone(),
            })
        }

        Self {
            messages: new_messages,
        }
    }
}
