use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub name: String
}

impl Marketplace {
    pub const LEN: usize = 8 + 32 + 2 + 4 + 32;
}

#[account]
pub struct Whitelist {
    pub collection_mint: Pubkey
}

impl Whitelist {
    pub const LEN: usize = 8 + 32;
}

#[account]
pub struct Listing {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub price: u64
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 32 + 8;
}