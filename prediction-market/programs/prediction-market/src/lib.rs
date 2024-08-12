use anchor_lang::prelude::*;

declare_id!("G3GnkQNmF8dykbf2jZ4BpMxuVztAv6gEsqdPrjc3Cme5");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
