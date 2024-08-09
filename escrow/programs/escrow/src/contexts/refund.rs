use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer=maker,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close=maker,
        has_one=mint_a,
        has_one=maker,
        seeds=[b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump=escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            authority: self.escrow.to_account_info(),
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata.to_account_info(),
        };
        let maker_key = self.maker.key();
        let escrow_seed = self.escrow.seed.to_be_bytes();
        let signer_seeds = &[
            b"escrow",
            maker_key.as_ref(),
            &escrow_seed[..],
            &[self.escrow.bump],
        ];

        let signer_seeds_arr = [&signer_seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds_arr,
        );

        transfer_checked(ctx, self.escrow.recive_amount, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds_arr,
        );

        close_account(ctx)
    }
}
