use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken ,token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked}
};

use crate::{state::Listing, MarketPlace};

#[derive(Accounts)]
pub struct DeList<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=user
    )]
    pub maker_nft_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds=[b"market_place",marketplace.name.as_str().as_bytes().as_ref()],
        bump=marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,
    #[account(
        mut,
        seeds=[b"listing",user.key().as_ref(),marketplace.key().as_ref()],
        bump=listing.bump,
        close=user
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=listing
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> DeList<'info> {
   pub fn withdraw_nft(&mut self)->Result<()>{

        let seeds=&[
                b"listing",
                &self.user.key().to_bytes()[..],
                &self.marketplace.key().to_bytes()[..],
                &[self.listing.bump]
        ];
        let signer_seeds=&[&seeds[..]];

        let accounts=TransferChecked{
                from:self.nft_vault.to_account_info(),
                to:self.maker_nft_ata.to_account_info(),
                authority:self.listing.to_account_info(),
                mint:self.nft_mint.to_account_info()
        };

        let ctx=CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        transfer_checked(ctx, 1, self.nft_mint.decimals)

   }
}
