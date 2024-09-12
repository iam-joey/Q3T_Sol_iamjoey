pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9W3fUnyS64hHTNQMFZ6WjHWTi518RytLzcNZ6v7sWU41");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<IntializeStakePool>) -> Result<()> {
        // initialize::handler(ctx)
        Ok(())
    }
}
