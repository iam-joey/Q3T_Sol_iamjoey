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
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        close=maker,
        has_one=maker,
        has_one=mint_a,
        has_one=mint_b,
        seeds=[b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Take<'info> {
    pub fn send_to_maker(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, self.escrow.recive_amount, self.mint_b.decimals)
    }

    pub fn vault_to_taker(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
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

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.taker.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds_arr,
        );

        close_account(ctx)
    }
}
