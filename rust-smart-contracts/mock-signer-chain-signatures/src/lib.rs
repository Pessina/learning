mod lib {
    pub mod signer;
}

use std::io::Read;

use ethers_core::k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::{generic_array::GenericArray, scalar::FromUintUnchecked},
    sha2::{Digest, Sha256},
    AffinePoint, Scalar, U256,
};
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

pub fn derive_epsilon(predecessor: String, path: String) -> Scalar {
    let mut hasher = Sha256::new();
    hasher.update(format!(
        "near-mpc-recovery v0.1.0 epsilon derivation:{predecessor},{path}"
    ));

    Scalar::from_uint_unchecked(U256::from_le_slice(&hasher.finalize()))
}

pub fn derive_public_key(
    public_key: &VerifyingKey,
    predecessor: String,
    path: String,
) -> VerifyingKey {
    let epsilon = derive_epsilon(predecessor, path);

    let new_public_key = (AffinePoint::GENERATOR * epsilon + public_key.as_affine()).to_affine();
    VerifyingKey::from_affine(new_public_key).expect("Invalid public key")
}

pub fn derive_private_key(
    private_key: &SigningKey,
    predecessor: String,
    path: String,
) -> SigningKey {
    let epsilon = derive_epsilon(predecessor, path);
    let new_private_key = epsilon.add(&private_key.as_nonzero_scalar());
    SigningKey::from_bytes(&new_private_key.to_bytes()).unwrap()
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

        let generic_array = GenericArray::from_slice(&self.seed);
        let signing_key = SigningKey::from_bytes(&generic_array).unwrap();

        let signing_key =
            derive_private_key(&signing_key, predecessor.to_string(), path.to_string());
        let (sig, recid) = signing_key.sign_prehash_recoverable(&payload).unwrap();
        PromiseOrValue::Value(MpcSignature::from_ecdsa_signature(sig, recid).unwrap())
    }

    fn public_key(&self) -> PublicKey {
        let wallet = LocalWallet::from_bytes(&self.seed).expect("Invalid seed");
        let public_key = wallet.signer().verifying_key();
        let encoded_point = public_key.to_encoded_point(false);
        let slice = &encoded_point.as_bytes()[1..65];
        let mut data = vec![near_sdk::CurveType::SECP256K1 as u8];
        data.extend(slice.to_vec());
        PublicKey::try_from(data).unwrap()
    }

    fn latest_key_version(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod test {
    use std::ops::{Deref, Mul};

    use super::*;

    use ethers_core::utils::hex::{self, ToHexExt};
    use ethers_core::{
        k256::{elliptic_curve::sec1::ToEncodedPoint, AffinePoint},
        utils::keccak256,
    };
    use ethers_signers::Signer;

    #[tokio::test]
    async fn test_kdf() {
        let account_id = "felipe-sandbox.testnet";
        let path = "btc";
        let wallet = LocalWallet::from_bytes(&[
            0xd5, 0x7b, 0xf9, 0x1c, 0xa3, 0xe6, 0x2d, 0x7c, 0x8f, 0x4e, 0xb3, 0x5a, 0x1f, 0x9d,
            0x6c, 0x2e, 0x8b, 0x4a, 0x7f, 0x3d, 0x9e, 0x5c, 0x1b, 0x6a, 0x2f, 0x8d, 0x4c, 0x7e,
            0x3b, 0x9a, 0x5f, 0x1d,
        ])
        .expect("Invalid seed");

        let public_key = wallet.signer().verifying_key();

        let new_public_key =
            derive_public_key(public_key, account_id.to_string(), path.to_string())
                .to_encoded_point(false);

        let new_private_key =
            derive_private_key(&wallet.signer(), account_id.to_string(), path.to_string());

        assert_eq!(
            (AffinePoint::GENERATOR.mul(new_private_key.as_nonzero_scalar().deref()))
                .to_encoded_point(false),
            new_public_key
        );

        let message = "Hello";
        let new_wallet = LocalWallet::from_bytes(&new_private_key.to_bytes()).unwrap();

        let signed_message = new_wallet.sign_message(message).await.unwrap();

        let address = signed_message.recover(message).unwrap();

        let public_key_bytes = &new_public_key.as_bytes()[1..];
        let public_key_hash = keccak256(public_key_bytes);

        let new_address = &public_key_hash[12..];
        let new_address = hex::encode(new_address);

        assert_eq!(address.encode_hex(), new_address);
    }
}
