use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{transfer, Transfer},
};

declare_id!("7A5DbBcNtukGesiLQ9vRp2Mk5STwxhFtBM13TVyL8imq");

#[program]
pub mod vault {
    use super::*;

    pub fn initalize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.save(ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+Vault::INIT_SPACE,
        seeds=[b"vault",user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn save(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault.set_inner(Vault { bump: bumps.vault });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault",user.key().as_ref()],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount * LAMPORTS_PER_SOL)
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault",user.key().as_ref()],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let rent = Rent::get()?;
        let min_lamports = rent.minimum_balance(Vault::INIT_SPACE);

        if self.vault.to_account_info().lamports() < amount*LAMPORTS_PER_SOL + min_lamports {
            return Err(Errors::NotEnoughBalance.into());
        }

        self.vault.sub_lamports(amount*LAMPORTS_PER_SOL)?;
        self.user.add_lamports(amount*LAMPORTS_PER_SOL)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub bump: u8,
}


#[error_code]
pub enum Errors {
    #[msg("Not enough balance")]
    NotEnoughBalance,
}