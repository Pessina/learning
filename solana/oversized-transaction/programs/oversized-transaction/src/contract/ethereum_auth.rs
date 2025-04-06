// use anchor_lang::prelude::*;
// use hex;
// use k256::ecdsa::{RecoveryId, Signature as K256Signature, VerifyingKey};
// use sha3::{Digest, Keccak256};

// pub fn verify_ethereum_signature_impl(
//     eth_data: &WalletValidationData,
//     compressed_public_key: &str,
// ) -> Result<bool> {
//     let provided_pubkey = match decode_public_key(compressed_public_key) {
//         Ok(pk) => pk,
//         Err(e) => {
//             msg!("Failed to decode public key: {}", e);
//             return Ok(false);
//         }
//     };

//     let message_digest = match prepare_message(eth_data) {
//         Ok(digest) => digest,
//         Err(e) => {
//             msg!("Failed to prepare message: {}", e);
//             return Ok(false);
//         }
//     };

//     let (signature, recovery_id) = match create_signature(&eth_data.signature) {
//         Ok(sig) => sig,
//         Err(e) => {
//             msg!("Failed to create signature: {}", e);
//             return Ok(false);
//         }
//     };

//     let recovered_pubkey = match recover_public_key(message_digest, &signature, recovery_id) {
//         Ok(key) => key,
//         Err(e) => {
//             msg!("Failed to recover public key: {}", e);
//             return Ok(false);
//         }
//     };

//     Ok(recovered_pubkey == provided_pubkey)
// }

// fn prepare_message(eth_data: &WalletValidationData) -> Result<[u8; 32]> {
//     let message_len = eth_data.message.len();
//     let prefix = format!("\x19Ethereum Signed Message:\n{message_len}");

//     let mut hasher = Keccak256::new();
//     hasher.update(prefix.as_bytes());
//     hasher.update(eth_data.message.as_bytes());

//     let digest = hasher.finalize();
//     Ok(digest.into())
// }

// fn create_signature(signature: &str) -> Result<(K256Signature, RecoveryId)> {
//     let mut sig_bytes = [0u8; 65];
//     hex::decode_to_slice(
//         signature.strip_prefix("0x").unwrap_or(signature),
//         &mut sig_bytes,
//     )
//     .map_err(|_| ErrorCode::InvalidHexEncoding)?;

//     let (r_s_bytes, v_byte) = sig_bytes.split_at(64);
//     let v = v_byte[0];

//     let signature =
//         K256Signature::try_from(r_s_bytes).map_err(|_| ErrorCode::InvalidSignatureFormat)?;

//     let recovery_id = RecoveryId::try_from(if v >= 27 { v - 27 } else { v })
//         .map_err(|_| ErrorCode::InvalidRecoveryId)?;

//     Ok((signature, recovery_id))
// }

// fn recover_public_key(
//     digest: [u8; 32],
//     signature: &K256Signature,
//     recovery_id: RecoveryId,
// ) -> Result<[u8; 33]> {
//     let verifying_key = VerifyingKey::recover_from_prehash(&digest, signature, recovery_id)
//         .map_err(|_| ErrorCode::PublicKeyRecoveryFailed)?;

//     let encoded_point = verifying_key.to_encoded_point(true);
//     let bytes = encoded_point.as_bytes();

//     if bytes.len() != 33 {
//         return Err(ErrorCode::InvalidPublicKeyLength.into());
//     }

//     let mut compressed = [0u8; 33];
//     compressed.copy_from_slice(bytes);
//     Ok(compressed)
// }

// fn decode_public_key(key: &str) -> Result<[u8; 33]> {
//     let stripped = key.strip_prefix("0x").unwrap_or(key);
//     let bytes = hex::decode(stripped).map_err(|_| ErrorCode::InvalidHexEncoding)?;
//     if bytes.len() != 33 {
//         return Err(ErrorCode::InvalidPublicKeyLength.into());
//     }
//     let mut arr = [0u8; 33];
//     arr.copy_from_slice(&bytes);
//     Ok(arr)
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct WalletValidationData {
//     pub signature: String,
//     pub message: String,
// }

// #[derive(Accounts)]
// #[instruction(eth_data: WalletValidationData, compressed_public_key: String)]
// pub struct VerifyEthereumSignature<'info> {
//     pub payer: Signer<'info>,
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Invalid hex encoding in signature or public key")]
//     InvalidHexEncoding,
//     #[msg("Invalid signature length - expected 65 bytes")]
//     InvalidSignatureLength,
//     #[msg("Invalid signature format")]
//     InvalidSignatureFormat,
//     #[msg("Invalid recovery ID")]
//     InvalidRecoveryId,
//     #[msg("Failed to recover public key")]
//     PublicKeyRecoveryFailed,
//     #[msg("Signature verification failed")]
//     SignatureVerificationFailed,
//     #[msg("Public key mismatch")]
//     PublicKeyMismatch,
//     #[msg("Invalid public key length - expected 33 bytes")]
//     InvalidPublicKeyLength,
// }
