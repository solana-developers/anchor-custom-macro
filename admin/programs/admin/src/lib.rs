use anchor_lang::prelude::*;
mod admin_config;
use admin_config::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod admin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        admin_config::initialize(ctx)
    }

    pub fn update_auth(ctx: Context<UpdateAdminAccount>, new_value: Pubkey) -> Result<()> {
        admin_config::Config::update_auth(ctx, new_value)
    }

    pub fn update_bool(ctx: Context<UpdateAdminAccount>, new_value: bool) -> Result<()> {
        admin_config::Config::update_bool(ctx, new_value)
    }

    pub fn update_first_number(ctx: Context<UpdateAdminAccount>, new_value: u8) -> Result<()> {
        admin_config::Config::update_first_number(ctx, new_value)
    }

    pub fn update_second_number(ctx: Context<UpdateAdminAccount>, new_value: u64) -> Result<()> {
        admin_config::Config::update_second_number(ctx, new_value)
    }
}
