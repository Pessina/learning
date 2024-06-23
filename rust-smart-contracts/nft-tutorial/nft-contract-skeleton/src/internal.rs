use crate::*;
use near_sdk::CryptoHash;
use std::mem::size_of;

const ONE_YOCTONEAR: NearToken = NearToken::from_yoctonear(1);

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        NearToken::from_yoctonear(1),
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

pub(crate) fn refund_deposit(storage_used: u128) {
    let required_cost = env::storage_byte_cost().saturating_mul(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost
    );

    let refund = attached_deposit.saturating_sub(required_cost);

    if refund.gt(&ONE_YOCTONEAR) {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&account_id),
            })
        });

        tokens_set.insert(token_id);

        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .expect("Token should be owned by the sender");

        tokens_set.remove(token_id);

        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id)
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set)
        };
    }
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        memo: Option<String>,
    ) -> Token {
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        self.internal_add_token_to_owner(receiver_id, token_id);

        let new_token = Token {
            owner_id: receiver_id.clone(),
        };

        self.tokens_by_id.insert(&token_id, &new_token);

        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string())
        };

        let mut authorized_id = None;

        let nft_transfer_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: token.owner_id.to_string(),
                new_owner_id: receiver_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };

        env::log_str(&nft_transfer_log.to_string());

        token
    }
}
