use anchor_lang::{prelude::*, Discriminator};

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub auth: Pubkey,
    pub bool: bool,
    pub first_number: u8,
    pub second_number: u64,
}

impl Config {
    pub const LEN: usize = Config::DISCRIMINATOR.len() + Config::INIT_SPACE;
}
