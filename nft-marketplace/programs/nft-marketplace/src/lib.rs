pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7Fskpa1MJ7JFezzvwYuwom2iZ7xvuUDck4jMMyqdiiM9");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn intialize_marketplace(
        ctx: Context<InitializeMarketPlace>,
        name: String,
        fees: u16,
    ) -> Result<()> {
        ctx.accounts.initalize_marketplace(name, fees, &ctx.bumps)
    }

    pub fn list_nft(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_user_listing(price, &ctx.bumps)?;
        ctx.accounts.transafer_nft_vault()
    }

    pub fn delist_nft(ctx: Context<DeList>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }
}
