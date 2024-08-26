use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::error::MarketPlaceErrorCodes;
use crate::state::MarketPlace;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitializeMarketPlace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer=admin,
        space=MarketPlace::INIT_SPACE,
        seeds=[b"market_place",name.as_str().as_bytes().as_ref()],
        bump
    )]
    pub market_place: Account<'info, MarketPlace>,
    #[account(
        init,
        payer=admin,
        seeds=[b"rewards_mint",market_place.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=market_place
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds=[b"vault",market_place.key().as_ref()],
        bump
    )]
    pub treasury_vault: SystemAccount<'info>, //this is to store the sol fess which we take from the user
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeMarketPlace<'info> {
    pub fn initalize_marketplace(
        &mut self,
        name: String,
        fees: u16,
        bumps: &InitializeMarketPlaceBumps,
    ) -> Result<()> {
        require!(
            name.len() > 0 && name.len() < 33,
            MarketPlaceErrorCodes::NameIsTooLong
        );
        self.market_place.set_inner(MarketPlace {
            admin: self.admin.key(),
            fees,
            bump: bumps.market_place,
            treasury_bump: bumps.treasury_vault,
            mint_bump: bumps.rewards_mint,
            name,
        });
        Ok(())
    }
}
