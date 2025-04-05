use anchor_lang::prelude::*;

declare_id!("8pZ3UMcQGe6GpXBppbLBE4xQDf5qmfCkvCzTNvDDXx9w");

const DISCRIMINATOR_SIZE: usize = 8;
const PUBKEY_SIZE: usize = 32;
const U16_SIZE: usize = 2;
const VEC_PREFIX_SIZE: usize = 4;
const CHUNK_OVERHEAD: usize = U16_SIZE + 1 + VEC_PREFIX_SIZE; // index (u16) + is_stored (bool) + vec prefix
const MAX_CHUNK_SIZE: usize = 900;
const MAX_REALLOC_SIZE: usize = 10 * 1024; // 10KB Solana reallocation limit per tx
const INITIAL_CHUNKS: usize = 5; // Initial number of chunks to allocate

#[program]
pub mod oversized_transaction {
    use super::*;

    pub fn init_storage(
        ctx: Context<InitStorage>, 
        data_id: [u8; 32], 
        total_chunks: u16,
        data_hash: [u8; 32], 
    ) -> Result<()> {
        msg!("Initializing new storage account");
        let storage = &mut ctx.accounts.unified_storage;
        storage.data_id = data_id;
        storage.total_chunks = total_chunks;
        storage.data_hash = data_hash;
        storage.chunks_stored = 0;
        
        // Set initial capacity (up to 5 chunks)
        let initial_capacity = if total_chunks as usize <= INITIAL_CHUNKS {
            total_chunks as usize
        } else {
            INITIAL_CHUNKS
        };
        storage.chunks = vec![ChunkData::default(); initial_capacity];
        
        msg!("Initialized with capacity for {} of {} chunks", 
            initial_capacity, total_chunks);
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
        require!(chunk_data.len() <= MAX_CHUNK_SIZE, ErrorCode::ChunkTooLarge);
        
        let initial = ctx.accounts.unified_storage.chunks.is_empty();

        require!(!initial, ErrorCode::StorageNotInitialized);
        
        // Verify data consistency
        let storage = &ctx.accounts.unified_storage;
        
        require!(storage.data_id == data_id, ErrorCode::InvalidDataId);
        require!(storage.total_chunks == total_chunks, ErrorCode::InvalidTotalChunks);
        require!(storage.data_hash == data_hash, ErrorCode::InvalidDataHash);    
        require!(chunk_index < total_chunks, ErrorCode::InvalidChunkIndex);
        
        // Check if we need to expand the storage for this chunk
        let needs_expansion = {
            let storage = &ctx.accounts.unified_storage;
            chunk_index as usize >= storage.chunks.len()
        };
        
        if needs_expansion {
            resize_storage_account(
                &mut ctx.accounts.unified_storage,
                &ctx.accounts.system_program,
                &ctx.accounts.payer,
                chunk_index
            )?;
        }
        
        // Store the chunk data
        let storage = &mut ctx.accounts.unified_storage;
        
        // Verify we have enough capacity now
        if chunk_index as usize >= storage.chunks.len() {
            msg!("Chunk index {} exceeds current capacity of {}. Multiple transactions needed.", 
                chunk_index, storage.chunks.len());
            return err!(ErrorCode::NeedsMoreCapacity);
        }
        
        let was_already_stored = storage.chunks[chunk_index as usize].is_stored;
        
        storage.chunks[chunk_index as usize] = ChunkData {
            index: chunk_index,
            data: chunk_data.clone(),
            is_stored: true,
        };
        
        // Only increment count if this is a new chunk
        if !was_already_stored {
            storage.chunks_stored = storage.chunks_stored.saturating_add(1);
        }
        
        msg!("Stored chunk {}/{} with size {} bytes", 
            chunk_index + 1, total_chunks, storage.chunks[chunk_index as usize].data.len());
        Ok(())
    }

    pub fn retrieve_chunk(ctx: Context<RetrieveChunk>, chunk_index: u16) -> Result<Vec<u8>> {
        let storage = &ctx.accounts.unified_storage;
        
        require!(chunk_index < storage.total_chunks, ErrorCode::InvalidChunkIndex);
        
        if chunk_index as usize >= storage.chunks.len() {
            return err!(ErrorCode::ChunkNotAllocated);
        }
        
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

// Helper function to resize the storage account (respecting Solana's 10KB limit)
fn resize_storage_account<'info>(
    unified_storage: &mut Account<'info, UnifiedStorage>,
    system_program: &Program<'info, System>,
    payer: &Signer<'info>,
    chunk_index: u16
) -> Result<()> {
    let current_len = unified_storage.chunks.len();
    
    // Calculate target capacity
    let target_idx = chunk_index as usize;
    let chunks_needed = target_idx - current_len + 1;
    
    // Calculate how many chunks we can add within Solana's 10KB limit
    let space_per_chunk = CHUNK_OVERHEAD + MAX_CHUNK_SIZE;
    let max_chunks_in_10kb = MAX_REALLOC_SIZE / space_per_chunk;
    
    // Take the smaller value: chunks needed or max possible within 10KB
    let chunks_to_add = if chunks_needed <= max_chunks_in_10kb {
        chunks_needed
    } else {
        max_chunks_in_10kb
    };
    
    let new_capacity = current_len + chunks_to_add;
    msg!("Resizing storage from {} to {} chunks (+{} chunks)", 
        current_len, new_capacity, chunks_to_add);
    
    // Calculate size increase and new total size
    let size_increase = chunks_to_add * space_per_chunk;
    let old_size = unified_storage.to_account_info().data_len();
    let new_size = old_size + size_increase;
    
    // Perform the reallocation
    let storage_info = unified_storage.to_account_info();
    storage_info.realloc(new_size, false)?;
    
    // Ensure account is rent exempt after reallocation
    let rent = Rent::get()?;
    let min_rent = rent.minimum_balance(new_size);
    
    if storage_info.lamports() < min_rent {
        let lamports_needed = min_rent - storage_info.lamports();
        
        anchor_lang::system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: payer.to_account_info(),
                    to: storage_info,
                },
            ),
            lamports_needed,
        )?;
    }
    
    // Resize the chunks vector
    unified_storage.chunks.resize(new_capacity, ChunkData::default());
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(data_id: [u8; 32], total_chunks: u16, data_hash: [u8; 32])]
pub struct InitStorage<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = calculate_initial_space(total_chunks),
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
#[instruction(data_id: [u8; 32], chunk_index: u16, total_chunks: u16, data_hash: [u8; 32], chunk_data: Vec<u8>)]
pub struct StoreChunk<'info> {
    #[account(
        mut,
        seeds = [
            b"unified_storage", 
            payer.key().as_ref(),
            &data_id
        ],
        bump,
        realloc = unified_storage.to_account_info().data_len(),
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub unified_storage: Account<'info, UnifiedStorage>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Helper function to calculate initial space for account creation
fn calculate_initial_space(total_chunks: u16) -> usize {
    let initial_chunks = if total_chunks as usize <= INITIAL_CHUNKS {
        total_chunks as usize 
    } else { 
        INITIAL_CHUNKS 
    };
    
    // Base size + space for initial chunks
    DISCRIMINATOR_SIZE + 
    PUBKEY_SIZE +          // data_id
    U16_SIZE +             // total_chunks
    U16_SIZE +             // chunks_stored
    PUBKEY_SIZE +          // data_hash
    VEC_PREFIX_SIZE +      // chunks vector prefix
    (initial_chunks * (CHUNK_OVERHEAD + MAX_CHUNK_SIZE))
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
    
    #[msg("Chunk not yet allocated - need more capacity")]
    ChunkNotAllocated,
    
    #[msg("Invalid data ID")]
    InvalidDataId,
    
    #[msg("Invalid total chunks")]
    InvalidTotalChunks,
    
    #[msg("Invalid data hash")]
    InvalidDataHash,
    
    #[msg("Chunk too large (exceeds maximum size)")]
    ChunkTooLarge,
    
    #[msg("Account needs more capacity - send more transactions")]
    NeedsMoreCapacity,

    #[msg("Storage not initialized")]
    StorageNotInitialized,
}
