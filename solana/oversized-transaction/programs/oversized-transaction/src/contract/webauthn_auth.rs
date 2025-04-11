use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, msg, program::invoke, pubkey::Pubkey};
use hex;
use sha2::{Digest, Sha256};
use std::str::FromStr;

/// Data structure for WebAuthN validation inputs.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WebauthnValidationData {
    pub signature: String,
    pub authenticator_data: String,
    pub client_data: String,
}

/// Verifies a WebAuthN signature using the secp256r1 program.
///
/// # Arguments
/// * `webauthn_data` - The WebAuthN validation data containing signature, authenticator data, and client data.
/// * `compressed_public_key` - The compressed public key in hex format (33 bytes).
///
/// # Returns
/// * `Result<bool>` - Returns `Ok(true)` if verification succeeds, or an error if it fails.
pub fn verify_webauthn_signature_impl(
    webauthn_data: &WebauthnValidationData,
    compressed_public_key: String,
) -> Result<bool> {
    // Define the secp256r1 program ID
    let secp256r1_program_id =
        Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();

    // Decode and validate the signature (must be 64 bytes)
    let signature_bytes = decode_hex(&webauthn_data.signature)?;
    if signature_bytes.len() != 64 {
        return Err(ErrorCode::InvalidSignatureLength.into());
    }

    // Decode authenticator data
    let authenticator_data_bytes = decode_hex(&webauthn_data.authenticator_data)?;

    // Compute SHA-256 hash of client data
    let mut hasher = Sha256::new();
    hasher.update(webauthn_data.client_data.as_bytes());
    let client_data_hash = hasher.finalize();

    // Construct the message: authenticator_data + SHA-256(client_data)
    let mut message = Vec::with_capacity(authenticator_data_bytes.len() + 32);
    message.extend_from_slice(&authenticator_data_bytes);
    message.extend_from_slice(&client_data_hash);

    // Decode and validate the compressed public key (must be 33 bytes)
    let compressed_public_key_bytes = decode_hex(&compressed_public_key)?;
    if compressed_public_key_bytes.len() != 33 {
        return Err(ErrorCode::InvalidPublicKeyLength.into());
    }

    // Define instruction data layout constants
    let num_signatures = 1u8;
    let padding = 0u8;
    let header_size = 1 + 1 + 14; // num_signatures (1) + padding (1) + offsets (14)
    let data_start = header_size as usize; // 16

    // Calculate offsets
    let signature_offset = data_start;
    let public_key_offset = signature_offset + 64;
    let message_offset = public_key_offset + 33;

    // Create offsets struct for one signature
    let offsets = Secp256r1SignatureOffsets {
        signature_offset: signature_offset as u16,
        signature_instruction_index: 0,
        public_key_offset: public_key_offset as u16,
        public_key_instruction_index: 0,
        message_data_offset: message_offset as u16,
        message_data_size: message.len() as u16,
        message_instruction_index: 0,
    };

    // Build instruction data
    let mut instruction_data = Vec::with_capacity(header_size + 64 + 33 + message.len());
    instruction_data.push(num_signatures);
    instruction_data.push(padding);
    instruction_data.extend_from_slice(&offsets.to_le_bytes());
    instruction_data.extend_from_slice(&signature_bytes);
    instruction_data.extend_from_slice(&compressed_public_key_bytes);
    instruction_data.extend_from_slice(&message);

    // Create and invoke the instruction
    let instruction = Instruction::new_with_bytes(secp256r1_program_id, &instruction_data, vec![]);
    match invoke(&instruction, &[]) {
        Ok(_) => {
            msg!("Signature verification succeeded");
            Ok(true)
        }
        Err(err) => {
            msg!("Signature verification failed with error: {:?}", err);
            Err(ErrorCode::SignatureVerificationFailed.into())
        }
    }
}

/// Decodes a hex string into a byte vector.
///
/// # Arguments
/// * `s` - The hex string (with or without "0x" prefix).
///
/// # Returns
/// * `Result<Vec<u8>>` - The decoded bytes or an error if the format is invalid.
fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim_start_matches("0x");
    hex::decode(s).map_err(|_| ErrorCode::InvalidSignatureFormat.into())
}

/// Struct representing the offsets for secp256r1 signature verification.
#[repr(C)]
struct Secp256r1SignatureOffsets {
    signature_offset: u16,
    signature_instruction_index: u16,
    public_key_offset: u16,
    public_key_instruction_index: u16,
    message_data_offset: u16,
    message_data_size: u16,
    message_instruction_index: u16,
}

impl Secp256r1SignatureOffsets {
    /// Serializes the struct into a 14-byte little-endian array.
    fn to_le_bytes(&self) -> [u8; 14] {
        let mut bytes = [0u8; 14];
        bytes[0..2].copy_from_slice(&self.signature_offset.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.signature_instruction_index.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.public_key_offset.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.public_key_instruction_index.to_le_bytes());
        bytes[8..10].copy_from_slice(&self.message_data_offset.to_le_bytes());
        bytes[10..12].copy_from_slice(&self.message_data_size.to_le_bytes());
        bytes[12..14].copy_from_slice(&self.message_instruction_index.to_le_bytes());
        bytes
    }
}

/// Custom error codes for the program.
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid signature format")]
    InvalidSignatureFormat,
    #[msg("Signature length must be 64 bytes")]
    InvalidSignatureLength,
    #[msg("Public key must be 33 bytes")]
    InvalidPublicKeyLength,
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,
}

#[derive(Accounts)]
pub struct VerifyWebauthnSignature {}
