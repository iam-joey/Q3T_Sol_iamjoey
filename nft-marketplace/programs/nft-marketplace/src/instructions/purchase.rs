use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked,CloseAccount,close_account}};

use crate::{state::Listing, MarketPlace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub user:UncheckedAccount<'info>,
    pub nft_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=nft_mint,
        associated_token::authority=taker,
    )]
    pub taker_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(
        seeds=[b"market_place",marketplace.name.as_str().as_bytes().as_ref()],
        bump=marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,
    #[account(
        mut,
        seeds=[b"listing",user.key().as_ref(),marketplace.key().as_ref()],
        bump=listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=listing
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program:Program<'info,System>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>
}


impl<'info> Purchase<'info>  {
    pub fn withdraw_nft(&mut self)->Result<()>{



        Ok(())
    }
}