use anchor_lang::prelude::*;

declare_id!("8pZ3UMcQGe6GpXBppbLBE4xQDf5qmfCkvCzTNvDDXx9w");

#[program]
pub mod oversized_transaction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn store_chunk(
        ctx: Context<StoreChunk>, 
        data_id: [u8; 32], 
        chunk_index: u16, 
        total_chunks: u16,
        data_hash: [u8; 32], 
        chunk_data: Vec<u8>
    ) -> Result<()> {
        let storage = &mut ctx.accounts.unified_storage;
        
        if chunk_index == 0 {
            storage.data_id = data_id;
            storage.total_chunks = total_chunks;
            storage.data_hash = data_hash;
            storage.chunks_stored = 0;
            storage.chunks = vec![ChunkData::default(); total_chunks as usize];
        } else {
            require!(storage.data_id == data_id, ErrorCode::InvalidDataId);
            require!(storage.total_chunks == total_chunks, ErrorCode::InvalidTotalChunks);
            require!(storage.data_hash == data_hash, ErrorCode::InvalidDataHash);
        }
        
        require!(chunk_index < total_chunks, ErrorCode::InvalidChunkIndex);
        storage.chunks[chunk_index as usize] = ChunkData {
            index: chunk_index,
            data: chunk_data,
            is_stored: true,
        };
        
        storage.chunks_stored = storage.chunks_stored.saturating_add(1);
        
        msg!("Stored chunk {}/{} with size {} bytes", 
            chunk_index + 1, total_chunks, storage.chunks[chunk_index as usize].data.len());
        Ok(())
    }

    pub fn retrieve_chunk(ctx: Context<RetrieveChunk>, chunk_index: u16) -> Result<Vec<u8>> {
        let storage = &ctx.accounts.unified_storage;
        
        require!(chunk_index < storage.total_chunks, ErrorCode::InvalidChunkIndex);
        require!(storage.chunks[chunk_index as usize].is_stored, ErrorCode::ChunkNotStored);
        
        Ok(storage.chunks[chunk_index as usize].data.clone())
    }

    pub fn get_data_metadata(ctx: Context<GetDataMetadata>) -> Result<DataMetadata> {
        let storage = &ctx.accounts.unified_storage;
        
        Ok(DataMetadata {
            data_id: storage.data_id,
            total_chunks: storage.total_chunks,
            chunks_stored: storage.chunks_stored,
            data_hash: storage.data_hash,
        })
    }
    
    pub fn close_storage(ctx: Context<CloseStorage>) -> Result<()> {
        msg!("Closed storage account. Freed {} chunks", ctx.accounts.unified_storage.chunks_stored);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(data_id: [u8; 32], chunk_index: u16, total_chunks: u16, data_hash: [u8; 32], chunk_data: Vec<u8>)]
pub struct StoreChunk<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + 2 + 2 + 32 + 2 + 4 + 
            (total_chunks as usize * (2 + 1 + 4 + 1024)), // Each chunk has index, is_stored flag, vec len, and estimated data size
        seeds = [
            b"unified_storage", 
            payer.key().as_ref(),
            &data_id
        ],
        bump
    )]
    pub unified_storage: Account<'info, UnifiedStorage>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(chunk_index: u16)]
pub struct RetrieveChunk<'info> {
    #[account(
        seeds = [
            b"unified_storage", 
            payer.key().as_ref(),
            &unified_storage.data_id
        ],
        bump
    )]
    pub unified_storage: Account<'info, UnifiedStorage>,
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetDataMetadata<'info> {
    #[account(
        seeds = [
            b"unified_storage", 
            payer.key().as_ref(),
            &unified_storage.data_id
        ],
        bump
    )]
    pub unified_storage: Account<'info, UnifiedStorage>,
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseStorage<'info> {
    #[account(
        mut,
        seeds = [
            b"unified_storage", 
            payer.key().as_ref(),
            &unified_storage.data_id
        ],
        bump,
        close = payer
    )]
    pub unified_storage: Account<'info, UnifiedStorage>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct UnifiedStorage {
    pub data_id: [u8; 32],       // Unique identifier for this dataset
    pub total_chunks: u16,       // Total number of chunks in this dataset
    pub chunks_stored: u16,      // Number of chunks stored so far
    pub data_hash: [u8; 32],     // Hash of the entire dataset for integrity verification
    pub chunks: Vec<ChunkData>,  // Vector of all chunks
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ChunkData {
    pub index: u16,              // Index of this chunk (0-based)
    pub is_stored: bool,         // Whether this chunk has been stored
    pub data: Vec<u8>,           // Chunk data
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DataMetadata {
    pub data_id: [u8; 32],
    pub total_chunks: u16,
    pub chunks_stored: u16,
    pub data_hash: [u8; 32],
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid chunk index")]
    InvalidChunkIndex,
    
    #[msg("Chunk not stored")]
    ChunkNotStored,
    
    #[msg("Invalid data ID")]
    InvalidDataId,
    
    #[msg("Invalid total chunks")]
    InvalidTotalChunks,
    
    #[msg("Invalid data hash")]
    InvalidDataHash,
}
