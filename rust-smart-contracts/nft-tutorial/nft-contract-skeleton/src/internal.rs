use crate::*;
use near_sdk::CryptoHash;
use std::mem::size_of;

const ONE_YOCTONEAR: NearToken = NearToken::from_yoctonear(1);

pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u128 {
    account_id.as_str().len() as u128 + 4 + size_of::<u128>() as u128
}

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

pub(crate) fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_account_ids: I,
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    let storage_release = approved_account_ids
        .map(bytes_for_approved_account_id)
        .sum();

    Promise::new(account_id).transfer(env::storage_byte_cost().saturating_mul(storage_release))
}

pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u32>,
) -> Promise {
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

pub(crate) fn royalty_to_payout(royalty_percentage: u128, amount_to_pay: u128) -> U128 {
    U128(
        amount_to_pay
            .saturating_mul(royalty_percentage)
            .saturating_div(10000),
    )
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
        approval_id: Option<u32>,
        memo: Option<String>,
    ) -> Token {
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        if &token.owner_id != receiver_id {
            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }

            if let Some(enforced_approval_id) = approval_id {
                let actual_approval_id = token
                    .approved_account_ids
                    .get(sender_id)
                    .expect("Sender is not approved account");

                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval_id {} is different from the given approval_id {}",
                    actual_approval_id, enforced_approval_id,
                )
            }
        }

        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        self.internal_add_token_to_owner(receiver_id, token_id);

        let new_token = Token {
            owner_id: receiver_id.clone(),
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
            royalty: token.royalty.clone(),
        };

        self.tokens_by_id.insert(&token_id, &new_token);

        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string())
        };

        let mut authorized_id = None;

        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }

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
