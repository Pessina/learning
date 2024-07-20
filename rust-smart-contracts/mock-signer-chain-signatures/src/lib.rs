mod lib {
    pub mod signer;
}

use ethers_signers::LocalWallet;
use lib::signer::{MpcSignature, SignerInterface};
use near_sdk::{
    env::{self, sha256},
    near, require, PanicOnDefault, PromiseOrValue, PublicKey,
};

#[must_use]
pub fn construct_spoof_key(
    predecessor: &[u8],
    path: &[u8],
) -> ethers_core::k256::ecdsa::SigningKey {
    let predecessor_hash = sha256([predecessor, b",", path].concat().as_slice());
    ethers_core::k256::ecdsa::SigningKey::from_bytes(predecessor_hash.as_slice().into()).unwrap()
}

#[derive(Debug, PanicOnDefault)]
#[near(contract_state)]
pub struct MockSignerContract {
    seed: [u8; 32],
}

#[near]
impl MockSignerContract {
    #[init]
    #[private]
    pub fn new() -> Self {
        MockSignerContract {
            seed: [
                0xd5, 0x7b, 0xf9, 0x1c, 0xa3, 0xe6, 0x2d, 0x7c, 0x8f, 0x4e, 0xb3, 0x5a, 0x1f, 0x9d,
                0x6c, 0x2e, 0x8b, 0x4a, 0x7f, 0x3d, 0x9e, 0x5c, 0x1b, 0x6a, 0x2f, 0x8d, 0x4c, 0x7e,
                0x3b, 0x9a, 0x5f, 0x1d,
            ],
        }
    }
}

#[near]
impl SignerInterface for MockSignerContract {
    #[payable]
    fn sign(
        &mut self,
        payload: [u8; 32],
        path: &String,
        key_version: u32,
    ) -> PromiseOrValue<MpcSignature> {
        require!(key_version == 0, "Key version not supported");
        let predecessor = env::predecessor_account_id();
        // This is unused, but needs to be in the sign signature.
        let signing_key = construct_spoof_key(predecessor.as_bytes(), path.as_bytes());
        let (sig, recid) = signing_key.sign_prehash_recoverable(&payload).unwrap();
        PromiseOrValue::Value(MpcSignature::from_ecdsa_signature(sig, recid).unwrap())
    }

    fn public_key(&self) -> PublicKey {
        let wallet = LocalWallet::from_bytes(&self.seed).expect("Invalid seed");
        let signing_key = wallet.signer();

        signing_key.verifying_key().to_sec1_bytes();

        "secp256k1:37aFybhUHCxRdDkuCcB3yHzxqK7N8EQ745MujyAQohXSsYymVeHzhLxKvZ2qYeRHf3pGFiAsxqFJZjpF9gP2JV5u".parse().unwrap()
    }

    fn latest_key_version(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ethers_core::k256::{
        elliptic_curve::{scalar::FromUintUnchecked, sec1::ToEncodedPoint},
        sha2::{Digest, Sha256},
        AffinePoint, Scalar, U256,
    };
    use ethers_core::utils::hex;

    #[tokio::test]
    async fn test_public_key() {
        let account_id = "felipe-sandbox.testnet";
        let path = "btc";
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "near-mpc-recovery v0.1.0 epsilon derivation:{account_id},{path}"
        ));

        let hash_int = Scalar::from_uint_unchecked(U256::from_le_slice(&hasher.finalize()));

        let wallet = LocalWallet::from_bytes(&[
            0xd5, 0x7b, 0xf9, 0x1c, 0xa3, 0xe6, 0x2d, 0x7c, 0x8f, 0x4e, 0xb3, 0x5a, 0x1f, 0x9d,
            0x6c, 0x2e, 0x8b, 0x4a, 0x7f, 0x3d, 0x9e, 0x5c, 0x1b, 0x6a, 0x2f, 0x8d, 0x4c, 0x7e,
            0x3b, 0x9a, 0x5f, 0x1d,
        ])
        .expect("Invalid seed");

        let public_key = wallet.signer().verifying_key();

        println!(
            "old_public_key: 04{:?}{:?}",
            hex::encode(public_key.to_encoded_point(false).x().unwrap()),
            hex::encode(public_key.to_encoded_point(false).y().unwrap())
        );

        let new_public_key =
            (AffinePoint::GENERATOR * hash_int + public_key.as_affine()).to_encoded_point(false);

        println!(
            "new_public_key: 04{:?}{:?}",
            hex::encode(new_public_key.x().unwrap()),
            hex::encode(new_public_key.y().unwrap())
        );
    }
}
