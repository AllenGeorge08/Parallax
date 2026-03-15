use crate::state::Oracle;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Oracle>(),
        seeds = [b"oracle",seed.to_le_bytes().as_ref()],
        bump
    )]
    pub oracle: Box<Account<'info, Oracle>>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_oracle(
        &mut self,
        seed: u64,
        price_exponent: i32,
        price_mantissa: i64,
        confidence: u64,
        is_valid: bool,
        bump: &InitializeBumps,
    ) -> Result<()> {
        let oracle = &mut self.oracle;
        oracle.is_valid = is_valid;
        oracle.price_exponent = price_exponent;
        oracle.price_mantissa = price_mantissa;
        oracle.bump = bump.oracle;
        oracle.confidence = confidence;

        let clock = Clock::get()?;

        oracle.last_update_epoch = clock.epoch;
        oracle.last_update_slot = clock.slot;
        oracle.authority = self.authority.key();
        Ok(())
    }
}
