use anchor_lang::prelude::*;

mod contract;

// use crate::contract::ethereum_auth::*;
use crate::contract::oidc_auth::*;
use crate::contract::transaction_buffer::*;

declare_id!("8pZ3UMcQGe6GpXBppbLBE4xQDf5qmfCkvCzTNvDDXx9w");

#[program]
pub mod oversized_transaction {
    use super::*;

    pub fn init_storage(
        ctx: Context<InitStorage>,
        data_id: [u8; 32],
        chunk_index: u16,
        total_chunks: u16,
        data_hash: [u8; 32],
        chunk_data: Vec<u8>,
    ) -> Result<()> {
        init_storage_impl(
            ctx,
            data_id,
            chunk_index,
            total_chunks,
            data_hash,
            chunk_data,
        )
    }

    pub fn store_chunk(
        ctx: Context<StoreChunk>,
        data_id: [u8; 32],
        chunk_index: u16,
        total_chunks: u16,
        data_hash: [u8; 32],
        chunk_data: Vec<u8>,
    ) -> Result<()> {
        store_chunk_impl(
            ctx,
            data_id,
            chunk_index,
            total_chunks,
            data_hash,
            chunk_data,
        )
    }

    pub fn retrieve_chunk(ctx: Context<RetrieveChunk>, chunk_index: u16) -> Result<Vec<u8>> {
        retrieve_chunk_impl(ctx, chunk_index)
    }

    pub fn get_data_metadata(ctx: Context<GetDataMetadata>) -> Result<DataMetadata> {
        get_data_metadata_impl(ctx)
    }

    pub fn close_storage(ctx: Context<CloseStorage>) -> Result<()> {
        close_storage_impl(ctx)
    }

    // pub fn verify_ethereum_signature(
    //     _ctx: Context<VerifyEthereumSignature>,
    //     eth_data: WalletValidationData,
    //     compressed_public_key: String,
    // ) -> Result<bool> {
    //     verify_ethereum_signature_impl(&eth_data, &compressed_public_key)
    // }

    pub fn verify_oidc_signature(_ctx: Context<VerifyOIDCSignature>) -> Result<bool> {
        verify_oidc_signature_impl()
    }
}
