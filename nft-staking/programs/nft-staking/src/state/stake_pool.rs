use anchor_lang::prelude::*;

#[account]
pub struct StakePool {
    pub bump: u8,
    pub max_nft_stake: u8,
    pub points_per_nft_stake: u8,
    pub freeze_period: u32,
    pub token_mint_bump: u8,
}

impl Space for StakePool {
    const INIT_SPACE: usize = 8 + 1 * 4 + 4;
}
