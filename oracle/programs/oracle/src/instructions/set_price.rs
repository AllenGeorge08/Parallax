use crate::state::Oracle;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct SetPrice<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
        seeds = [b"oracle",seed.to_le_bytes().as_ref()],
        bump
    )]
    pub oracle: Box<Account<'info, Oracle>>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetPrice<'info> {
    pub fn set_prices(
        &mut self,
        price_mantissa: i64,
        price_exponent: i32,
        confidence: u64,
    ) -> Result<()> {
        assert_eq!(
            self.authority.key(),
            self.oracle.authority,
            "Only Owner Can Set Prices"
        );
        let oracle = &mut self.oracle;
        oracle.price_exponent = price_exponent;
        oracle.price_mantissa = price_mantissa;
        oracle.confidence = confidence;

        let clock = Clock::get()?;
        oracle.last_update_slot = clock.slot;
        oracle.last_update_epoch = clock.epoch;

        Ok(())
    }
}
