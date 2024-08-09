use anchor_lang::prelude::*;

declare_id!("5xRRHN4qdAVkdJM7DzxuCaBdgYs8rCTt8xMWKwSj1LZy");

pub mod contexts;
pub use contexts::*;

pub mod state;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        recieve_amount: u64,
    ) -> Result<()> {
        ctx.accounts.deposit_to_vault(deposit_amount)?;
        ctx.accounts.save_escrow(ctx.bumps, recieve_amount, seed)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx:Context<Take>)->Result<()>{
        ctx.accounts.send_to_maker()?;
        ctx.accounts.vault_to_taker()
    }
}
