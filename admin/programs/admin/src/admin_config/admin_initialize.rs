use crate::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"admin"], bump, payer = authority, space = Config::LEN)]
    pub admin_account: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.admin_account.auth = ctx.accounts.authority.key();
        ctx.accounts.admin_account.bool = true;
        ctx.accounts.admin_account.first_number = 1;
        ctx.accounts.admin_account.second_number = 2;
        Ok(())
    }
}
