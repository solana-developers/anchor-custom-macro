use crate::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateAdminAccount<'info> {
    pub auth: Signer<'info>,
    #[account(
        mut,
        has_one = auth,
    )]
    pub admin_account: Account<'info, Config>,
}

impl UpdateAdminAccount<'_> {
    pub fn update_auth(ctx: Context<UpdateAdminAccount>, new_value: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.auth = new_value;
        Ok(())
    }
    pub fn update_bool(ctx: Context<UpdateAdminAccount>, new_value: bool) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.bool = new_value;
        Ok(())
    }
    pub fn update_first_number(ctx: Context<UpdateAdminAccount>, new_value: u8) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.first_number = new_value;
        Ok(())
    }
    pub fn update_second_number(ctx: Context<UpdateAdminAccount>, new_value: u64) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.second_number = new_value;
        Ok(())
    }
}
