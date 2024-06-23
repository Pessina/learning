use crate::*;

use self::nft_core::NonFungibleTokenCore;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U64 {
        U64::from(self.token_metadata_by_id.len())
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u32>) -> Vec<JsonToken> {
        let from = u128::from(from_index.unwrap_or(U128::from(0)));

        self.token_metadata_by_id
            .keys()
            .skip(from as usize)
            .take(u32::from(limit.unwrap_or(50)) as usize)
            .map(|token_id| self.nft_token(token_id).unwrap())
            .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U64 {
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U64::from(tokens_for_owner_set.len())
        } else {
            U64::from(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u32>,
    ) -> Vec<JsonToken> {
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128::from(0)));

        tokens
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id).unwrap())
            .collect()
    }
}
