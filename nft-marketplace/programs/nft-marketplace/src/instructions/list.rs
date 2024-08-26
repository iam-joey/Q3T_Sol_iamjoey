use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{MasterEditionAccount, Metadata, MetadataAccount}, token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked}
};

use crate::{state::Listing, MarketPlace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: InterfaceAccount<'info, Mint>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
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
        init,
        payer=user,
        space=Listing::INIT_SPACE,
        seeds=[b"listing",user.key().as_ref(),marketplace.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=nft_mint,
        associated_token::authority=listing
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[
                b"metadata",
                metadata_program.key().as_ref(),
                nft_mint.key().as_ref()
        ],
        bump,
        seeds::program=metadata_program.key(),
        constraint = collection_mint.key().as_ref()==metadata_account.collection.as_ref().unwrap().key.as_ref(),
        constraint = metadata_account.collection.as_ref().unwrap().verified==true
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    #[account(
        seeds=[
                b"metadata",
                metadata_program.key().as_ref(),
                nft_mint.key().as_ref(),
                b"edition"
        ],
        seeds::program=metadata_program.key(),
        bump,
    )]
    pub edition_account: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> List<'info> {
    pub fn create_user_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            owner: self.user.key(),
            nft_mint: self.nft_mint.key(),
            bump: bumps.listing,
            price,
        });
        Ok(())
    }

    pub fn transafer_nft_vault(&mut self)->Result<()>{
        let accounts=TransferChecked{
                from:self.maker_nft_ata.to_account_info(),
                to:self.nft_vault.to_account_info(),
                authority:self.user.to_account_info(),
                mint:self.nft_mint.to_account_info()
        };

        let ctx=CpiContext::new(self.token_program.to_account_info(), accounts);
        
        transfer_checked(ctx, 1, self.nft_mint.decimals)

    }
}
