use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, msg, program::invoke, pubkey::Pubkey};
use hex;
use sha2::{Digest, Sha256};
use std::str::FromStr;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WebauthnValidationData {
    pub signature: String,
    pub authenticator_data: String,
    pub client_data: String,
}

pub fn verify_webauthn_signature_impl(
    ctx: &Context<VerifyWebauthnSignature>,
    webauthn_data: &WebauthnValidationData,
    compressed_public_key: String,
) -> Result<bool> {
    let secp256r1_program_id =
        Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();

    let signature_bytes = decode_hex(&webauthn_data.signature)?;
    if signature_bytes.len() != 64 {
        return Err(ErrorCode::InvalidSignatureLength.into());
    }

    let authenticator_data_bytes = decode_hex(&webauthn_data.authenticator_data)?;

    let mut hasher = Sha256::new();
    hasher.update(webauthn_data.client_data.as_bytes());
    let client_data_hash = hasher.finalize();

    let mut message = Vec::with_capacity(authenticator_data_bytes.len() + 32);
    message.extend_from_slice(&authenticator_data_bytes);
    message.extend_from_slice(&client_data_hash);

    let compressed_public_key_bytes = decode_hex(&compressed_public_key)?;
    if compressed_public_key_bytes.len() != 33 {
        return Err(ErrorCode::InvalidPublicKeyLength.into());
    }

    let num_signatures = 1u8;
    let padding = 0u8;
    let header_size = 1 + 1 + 14;
    let data_start = header_size as usize;

    let signature_offset = data_start;
    let public_key_offset = signature_offset + 64;
    let message_offset = public_key_offset + 33;

    let offsets = Secp256r1SignatureOffsets {
        signature_offset: signature_offset as u16,
        signature_instruction_index: 0,
        public_key_offset: public_key_offset as u16,
        public_key_instruction_index: 0,
        message_data_offset: message_offset as u16,
        message_data_size: message.len() as u16,
        message_instruction_index: 0,
    };

    let mut instruction_data = Vec::with_capacity(header_size + 64 + 33 + message.len());
    instruction_data.push(num_signatures);
    instruction_data.push(padding);
    instruction_data.extend_from_slice(&offsets.to_le_bytes());
    instruction_data.extend_from_slice(&signature_bytes);
    instruction_data.extend_from_slice(&compressed_public_key_bytes);
    instruction_data.extend_from_slice(&message);

    let instruction = Instruction::new_with_bytes(secp256r1_program_id, &instruction_data, vec![]);
    match invoke(
        &instruction,
        &[ctx.accounts.instructions_sysvar.to_account_info()],
    ) {
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

fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim_start_matches("0x");
    hex::decode(s).map_err(|_| ErrorCode::InvalidSignatureFormat.into())
}

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
pub struct VerifyWebauthnSignature<'info> {
    /// CHECK: Skip security check
    #[account()]
    pub instructions_sysvar: AccountInfo<'info>,
}
