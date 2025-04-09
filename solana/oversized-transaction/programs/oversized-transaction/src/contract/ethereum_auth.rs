use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::{secp256k1_recover, Secp256k1Pubkey};
use hex;
use sha3::{Digest, Keccak256};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WalletValidationData {
    pub signature: String,
    pub message: String,
}

fn eth_signed_message_hash(message: String) -> [u8; 32] {
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());

    let mut hasher = Keccak256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(message.as_bytes());

    hasher.finalize().into()
}

fn normalize_key(key: &str) -> String {
    key.strip_prefix("0x").unwrap_or(key).to_ascii_lowercase()
}

pub fn verify_ethereum_signature_impl(
    eth_data: &WalletValidationData,
    compressed_public_key: &str,
) -> Result<bool> {
    let expected_pubkey = match decode_pubkey_from_hex(compressed_public_key) {
        Ok(pk) => {
            msg!("Successfully decoded public key");
            pk
        }
        Err(e) => {
            msg!("Failed to decode public key: {}", e);
            return Err(ErrorCode::InvalidPublicKeyLength.into());
        }
    };

    let message_hash = eth_signed_message_hash(eth_data.message.clone());

    let (signature_bytes, recovery_id) = match parse_signature(&eth_data.signature) {
        Ok(parsed) => {
            msg!("Successfully parsed signature, recovery_id: {}", parsed.1);
            parsed
        }
        Err(e) => {
            msg!("Failed to parse signature: {}", e);
            return Err(ErrorCode::InvalidSignatureFormat.into());
        }
    };

    msg!(
        "Attempting to recover public key with recovery_id: {}",
        recovery_id
    );

    let recovered_pubkey = match secp256k1_recover(&message_hash, recovery_id, &signature_bytes) {
        Ok(pubkey) => {
            msg!("Successfully recovered public key");
            pubkey
        }
        Err(e) => {
            msg!("Failed to recover public key: {:?}", e);
            return Err(ErrorCode::PublicKeyRecoveryFailed.into());
        }
    };

    let result = compare_pubkeys(&recovered_pubkey, &expected_pubkey);
    msg!("Signature verification result: {}", result);

    Ok(result)
}

fn decode_pubkey_from_hex(key: &str) -> Result<[u8; 32]> {
    let stripped = key.strip_prefix("0x").unwrap_or(key);
    msg!("Decoding public key (stripped): {}", stripped);

    let bytes = match hex::decode(stripped) {
        Ok(b) => {
            msg!("Successfully decoded hex, length: {}", b.len());
            b
        }
        Err(e) => {
            msg!("Failed to decode hex: {:?}", e);
            return Err(ErrorCode::InvalidHexEncoding.into());
        }
    };

    let mut arr = [0u8; 32];

    let actual_bytes = &bytes[1..];
    arr[..actual_bytes.len()].copy_from_slice(actual_bytes);

    Ok(arr)
}

fn parse_signature(signature: &str) -> Result<([u8; 64], u8)> {
    let sig_bytes = hex::decode(signature.strip_prefix("0x").unwrap_or(signature)).unwrap();

    let (r_s_bytes, v_byte) = sig_bytes.split_at(64);
    let v = v_byte[0];

    let signature = r_s_bytes;

    let recovery_id = if v >= 27 { v - 27 } else { v };

    Ok((signature.try_into().unwrap(), recovery_id))
}

fn compress_pubkey(pubkey: &[u8]) -> [u8; 33] {
    let mut compressed = [0u8; 33];

    let y_is_odd = pubkey[63] & 1 == 1;
    compressed[0] = if y_is_odd { 0x03 } else { 0x02 };

    compressed[1..33].copy_from_slice(&pubkey[0..32]);

    compressed
}

fn compare_pubkeys(recovered: &Secp256k1Pubkey, expected: &[u8; 32]) -> bool {
    let recovered_bytes = recovered.to_bytes();

    msg!("Recovered bytes: {:?}", recovered_bytes);
    msg!("Expected bytes: {:?}", expected);

    let compressed_recovered = compress_pubkey(&recovered_bytes[1..]);

    let mut expected_compressed = [0u8; 33];

    let copy_len = expected.len().min(32);
    expected_compressed[1..copy_len + 1].copy_from_slice(&expected[..copy_len]);

    expected_compressed[0] = 0x02;

    msg!("Compressed recovered: {:?}", compressed_recovered);
    msg!("Expected compressed: {:?}", expected_compressed);

    let matches = compressed_recovered == expected_compressed;
    msg!("Public keys match: {}", matches);

    matches
}

#[derive(Accounts)]
#[instruction(eth_data: WalletValidationData, compressed_public_key: String)]
pub struct VerifyEthereumSignature<'info> {
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid hex encoding in signature or public key")]
    InvalidHexEncoding,
    #[msg("Invalid signature length - expected 65 bytes hex encoded")]
    InvalidSignatureLength,
    #[msg("Invalid signature format")]
    InvalidSignatureFormat,
    #[msg("Invalid recovery ID")]
    InvalidRecoveryId,
    #[msg("Failed to recover public key")]
    PublicKeyRecoveryFailed,
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,
    #[msg("Public key mismatch")]
    PublicKeyMismatch,
    #[msg("Invalid public key length - expected 64 bytes")]
    InvalidPublicKeyLength,
    #[msg("Invalid message format - expected 32 byte hash")]
    InvalidMessageFormat,
}
