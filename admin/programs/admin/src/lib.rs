use anchor_lang::prelude::*;
mod admin_config;
use admin_config::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod admin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::initialize(ctx)
    }

    pub fn update_auth(ctx: Context<UpdateAdminAccount>, new_value: Pubkey) -> Result<()> {
        UpdateAdminAccount::update_auth(ctx, new_value)
    }

    pub fn update_bool(ctx: Context<UpdateAdminAccount>, new_value: bool) -> Result<()> {
        UpdateAdminAccount::update_bool(ctx, new_value)
    }

    pub fn update_first_number(ctx: Context<UpdateAdminAccount>, new_value: u8) -> Result<()> {
        UpdateAdminAccount::update_first_number(ctx, new_value)
    }

    pub fn update_second_number(ctx: Context<UpdateAdminAccount>, new_value: u64) -> Result<()> {
        UpdateAdminAccount::update_second_number(ctx, new_value)
    }
}
