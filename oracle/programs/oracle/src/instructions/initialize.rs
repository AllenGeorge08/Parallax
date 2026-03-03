use anchor_lang::prelude::*;
use crate::state::Oracle;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + std::mem::size_of::<Oracle>(),
        seeds = [b"oracle",seed.to_le_bytes().as_ref()],
        bump
    )]
    pub oracle: Box<Account<'info,Oracle>>,
    pub system_program: Program<'info,System>,
}

impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, seed: u64,price_exponent: i32,price_mantissa: i64,confidence: u64,last_update_slot: i64, last_update_epoch: u64,is_valid: bool, bump: &InitializeBumps) -> Result<()>{
        let oracle = &mut self.oracle;
        oracle.is_valid = is_valid;
        oracle.price_exponent = price_exponent;
        oracle.price_mantissa = price_mantissa;
        oracle.bump = bump.oracle;
        oracle.confidence = confidence;
        oracle.last_update_epoch = last_update_epoch;
        oracle.last_update_slot = last_update_slot;
        Ok(())
    }
}