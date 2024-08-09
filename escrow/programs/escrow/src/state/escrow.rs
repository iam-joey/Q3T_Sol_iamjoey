use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub bump: u8,
    pub maker: Pubkey,
    pub recive_amount: u64,
    pub seed: u64,
}
