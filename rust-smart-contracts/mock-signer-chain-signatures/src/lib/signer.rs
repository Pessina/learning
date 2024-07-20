use ethers_core::{
    k256::{
        ecdsa::RecoveryId,
        elliptic_curve::{group::GroupEncoding, point::DecompressPoint},
        AffinePoint,
    },
    utils::hex,
};
use near_sdk::{
    serde::{Deserialize, Serialize},
    PromiseOrValue, PublicKey,
};
use schemars::JsonSchema;

pub trait SignerInterface {
    fn sign(
        &mut self,
        payload: [u8; 32],
        path: &String,
        key_version: u32,
    ) -> PromiseOrValue<MpcSignature>;
    fn public_key(&self) -> PublicKey;
    fn latest_key_version(&self) -> u32;
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct MpcSignature(pub String, pub String);

impl MpcSignature {
    #[must_use]
    pub fn new(r: [u8; 32], s: [u8; 32], v: RecoveryId) -> Option<Self> {
        let big_r = Option::<AffinePoint>::from(AffinePoint::decompress(
            &r.into(),
            u8::from(v.is_y_odd()).into(),
        ))?;

        Some(Self(hex::encode(big_r.to_bytes()), hex::encode(s)))
    }

    #[must_use]
    pub fn from_ecdsa_signature(
        signature: ethers_core::k256::ecdsa::Signature,
        recovery_id: RecoveryId,
    ) -> Option<Self> {
        MpcSignature::new(
            signature.r().to_bytes().into(),
            signature.s().to_bytes().into(),
            recovery_id,
        )
    }
}
