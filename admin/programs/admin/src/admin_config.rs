use anchor_lang::prelude::*;
use custom_macro::Instructions;

#[derive(Instructions)]
#[account]
pub struct Config {
    auth: Pubkey,
    bool: bool,
    first_number: u8,
    second_number: u64,
}

impl Config {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 8;
}

#[derive(Accounts)]
pub struct UpdateAdminAccount<'info> {
    pub auth: Signer<'info>,
    #[account(
        mut,
        has_one = auth,
    )]
    pub admin_account: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"admin"], bump, payer = authority, space = Config::LEN)]
    pub admin_account: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.admin_account.auth = ctx.accounts.authority.key();
    ctx.accounts.admin_account.bool = true;
    ctx.accounts.admin_account.first_number = 1;
    ctx.accounts.admin_account.second_number = 2;
    Ok(())
}
