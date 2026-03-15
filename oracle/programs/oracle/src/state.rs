use anchor_lang::prelude::*;

#[account]
pub struct Oracle {
    pub authority: Pubkey,
    pub price_mantissa: i64,
    pub price_exponent: i32,
    pub confidence: u64,
    pub last_update_slot: u64,
    pub last_update_epoch: u64,
    pub is_valid: bool,
    pub bump: u8,
}
