use anchor_lang::prelude::*;

declare_id!("8pZ3UMcQGe6GpXBppbLBE4xQDf5qmfCkvCzTNvDDXx9w");

#[program]
pub mod oversized_transaction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
