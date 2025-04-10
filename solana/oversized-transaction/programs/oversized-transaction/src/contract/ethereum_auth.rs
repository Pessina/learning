use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use hex;
use sha3::{Digest, Keccak256};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WalletValidationData {
    pub signature: String,
    pub message: String,
}

fn eth_signed_message_hash(message: &str) -> [u8; 32] {
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    let mut hasher = Keccak256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(message.as_bytes());
    hasher.finalize().into()
}

pub fn verify_ethereum_signature_impl(
    eth_data: &WalletValidationData,
    compressed_public_key: &str,
) -> Result<bool> {
    let sig_str = if eth_data.signature.starts_with("0x") {
        &eth_data.signature[2..]
    } else {
        &eth_data.signature
    };
    let signature_bytes = hex::decode(sig_str).map_err(|_| ErrorCode::InvalidSignatureFormat)?;
    if signature_bytes.len() != 65 {
        return Err(ErrorCode::InvalidSignatureLength.into());
    }

    let sig = &signature_bytes[0..64];
    let v = signature_bytes[64];
    let recovery_id = if v >= 27 { v - 27 } else { v };
    if recovery_id > 3 {
        return Err(ErrorCode::InvalidRecoveryId.into());
    }

    let message_hash = eth_signed_message_hash(eth_data.message.as_str());

    let recovered_pubkey = secp256k1_recover(&message_hash, recovery_id, sig)
        .map_err(|_| ErrorCode::RecoveryFailed)?;

    let pk_str = if compressed_public_key.starts_with("0x") {
        &compressed_public_key[2..]
    } else {
        &compressed_public_key
    };
    let compressed_pubkey_bytes =
        hex::decode(pk_str).map_err(|_| ErrorCode::InvalidPublicKeyFormat)?;
    if compressed_pubkey_bytes.len() != 33 {
        return Err(ErrorCode::InvalidPublicKeyLength.into());
    }

    let provided_prefix = compressed_pubkey_bytes[0];
    if provided_prefix != 0x02 && provided_prefix != 0x03 {
        return Err(ErrorCode::InvalidPublicKeyFormat.into());
    }
    let provided_x = &compressed_pubkey_bytes[1..33];

    let x = &recovered_pubkey.0[0..32];
    let y = &recovered_pubkey.0[32..64];
    let prefix = if y[31] & 1 == 0 { 0x02 } else { 0x03 };

    if provided_prefix == prefix && provided_x == x {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Accounts)]
pub struct VerifyEthereumSignature {}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid signature format")]
    InvalidSignatureFormat,
    #[msg("Signature length must be 65 bytes")]
    InvalidSignatureLength,
    #[msg("Invalid recovery ID")]
    InvalidRecoveryId,
    #[msg("Failed to recover public key")]
    RecoveryFailed,
    #[msg("Invalid public key format")]
    InvalidPublicKeyFormat,
    #[msg("Public key length must be 33 bytes for compressed form")]
    InvalidPublicKeyLength,
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,
}
