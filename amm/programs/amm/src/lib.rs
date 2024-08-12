use anchor_lang::prelude::*;

declare_id!("E6Wc1PsvcDyMmRUtWkH9UWgcbJKjRHZVrjwMqxV2R4Hy");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
