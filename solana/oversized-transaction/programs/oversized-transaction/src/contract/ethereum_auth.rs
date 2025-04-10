use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use hex;
use libsecp256k1::{PublicKey, PublicKeyFormat};
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

pub fn verify_ethereum_signature_impl(
    eth_data: &WalletValidationData,
    compressed_public_key: &str,
) -> Result<bool> {
    // Log the message being verified
    msg!(
        "Verifying Ethereum signature for message: {}",
        eth_data.message
    );

    // Step 1: Compute the Ethereum signed message hash
    let message_hash = eth_signed_message_hash(eth_data.message.clone());
    msg!("Computed message hash: {:?}", message_hash);

    // Step 2: Decode the signature from hex string
    let signature_bytes = match hex::decode(&eth_data.signature.replace("0x", "")) {
        Ok(bytes) => bytes,
        Err(_) => {
            msg!("Error: Failed to decode signature hex string");
            return Ok(false);
        }
    };
    if signature_bytes.len() != 65 {
        msg!(
            "Error: Signature length is {}, expected 65 bytes",
            signature_bytes.len()
        );
        return Ok(false);
    }

    // Step 3: Extract r, s, and v from the signature
    let sig = &signature_bytes[0..64]; // First 64 bytes are r (32) + s (32)
    let v = signature_bytes[64]; // Last byte is the recovery ID (v)
                                 // Ethereum's v is typically 27 or 28; secp256k1_recover expects 0 or 1
    let recovery_id = if v >= 27 { v - 27 } else { v };
    if recovery_id > 3 {
        msg!("Error: Recovery ID {} is invalid, must be 0-3", recovery_id);
        return Ok(false);
    }
    msg!("Extracted recovery ID: {}", recovery_id);

    // Step 4: Recover the public key from the signature
    let recovered_pubkey = match secp256k1_recover(&message_hash, recovery_id, sig) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            msg!("Error: Failed to recover public key from signature");
            return Ok(false);
        }
    };
    msg!("Recovered public key: {:?}", recovered_pubkey.0);

    // Step 5: Decode the provided compressed public key
    let compressed_pubkey_bytes = match hex::decode(compressed_public_key.replace("0x", "")) {
        Ok(bytes) => bytes,
        Err(_) => {
            msg!("Error: Failed to decode compressed public key hex string");
            return Ok(false);
        }
    };
    if compressed_pubkey_bytes.len() != 33 {
        msg!(
            "Error: Compressed public key length is {}, expected 33 bytes",
            compressed_pubkey_bytes.len()
        );
        return Ok(false);
    }

    // Step 6: Parse and decompress the provided public key
    let public_key =
        match PublicKey::parse_slice(&compressed_pubkey_bytes, Some(PublicKeyFormat::Compressed)) {
            Ok(pk) => pk,
            Err(_) => {
                msg!("Error: Failed to parse compressed public key");
                return Ok(false);
            }
        };
    let serialized = public_key.serialize(); // 65 bytes with 0x04 prefix
    if serialized.len() != 65 || serialized[0] != 0x04 {
        msg!("Error: Unexpected serialized public key format");
        return Ok(false);
    }
    let uncompressed_pubkey = &serialized[1..65]; // Remove 0x04 prefix to get 64 bytes
    msg!(
        "Provided public key (uncompressed): {:?}",
        uncompressed_pubkey
    );

    // Step 7: Compare the recovered and provided public keys
    let is_valid = uncompressed_pubkey == &recovered_pubkey.0[..];
    msg!("Signature verification result: {}", is_valid);
    Ok(is_valid)
}

#[derive(Accounts)]
pub struct VerifyEthereumSignature {}

// Custom error codes for meaningful error messages
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
}
