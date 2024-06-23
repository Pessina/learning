use crate::{internal::refund_deposit, *};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        let initial_storage_usage = env::storage_usage();

        let token = Token {
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
        };

        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exist"
        );

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        let nft_mint_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: None,
            }]),
        };

        env::log_str(&nft_mint_log.to_string());

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes.into());
    }
}
