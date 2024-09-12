use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::StakePool;

#[derive(Accounts)]

pub struct IntializeStakePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer=creator,
        space=StakePool::INIT_SPACE,
        seeds=[b"stake_pool".as_ref()],
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        init,
        payer=creator,
        seeds=[b"rewards_mint".as_ref(),stake_pool.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=stake_pool
    )]
    pub rewards_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> IntializeStakePool<'info> {
    pub fn create_pool(
        &mut self,
        max_nft_stake: u8,
        points_per_nft_stake: u8,
        freeze_period: u32,
        bumps: IntializeStakePoolBumps,
    ) -> Result<()> {
        self.stake_pool.set_inner(StakePool {
            bump: bumps.stake_pool,
            max_nft_stake,
            points_per_nft_stake,
            freeze_period,
            token_mint_bump: bumps.rewards_mint,
        });
        Ok(())
    }
}
