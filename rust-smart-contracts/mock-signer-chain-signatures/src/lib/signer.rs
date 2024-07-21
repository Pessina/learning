use near_sdk::{PromiseOrValue, PublicKey};

use super::types::SignatureResponse;

pub trait SignerInterface {
    fn sign(
        &mut self,
        payload: [u8; 32],
        path: &String,
        key_version: u32,
    ) -> PromiseOrValue<SignatureResponse>;
    fn public_key(&self) -> PublicKey;
    fn latest_key_version(&self) -> u32;
}
