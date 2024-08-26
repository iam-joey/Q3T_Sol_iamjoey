use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub bump: u8,
    pub price: u64,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 * 2 + 1 + 8;
}
