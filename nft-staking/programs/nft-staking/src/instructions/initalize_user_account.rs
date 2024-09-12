use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitalizeUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        space=UserAccount::INIT_SPACE,
        seeds=[b"user_account".as_ref(),user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitalizeUserAccount<'info> {
    pub fn create_user(&mut self) -> Result<()> {
        Ok(())
    }
}
