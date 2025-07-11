use anchor_lang::prelude::*;

declare_id!("BbV3erud5jzA1mBui1MLw4n2yWeXtLCi6eYYSnxeMbD6");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
