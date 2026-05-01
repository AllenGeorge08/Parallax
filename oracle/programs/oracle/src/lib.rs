use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5oaExwo2hMmf4C7wHW1SGHjxbgthrZD8HcmAjdf5KNej");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        price_exponent: i32,
        price_mantissa: i64,
        confidence: u64,
        is_valid: bool,
    ) -> Result<()> {
        ctx.accounts.initialize_oracle(
            seed,
            price_exponent,
            price_mantissa,
            confidence,
            is_valid,
            &ctx.bumps,
        )?;
        Ok(())
    }

    pub fn set_price(
        ctx: Context<SetPrice>,
        seed: u64,
        price_mantissa: i64,
        price_exponent: i32,
        confidence: u64,
    ) -> Result<()> {
        ctx.accounts
            .set_prices(seed, price_mantissa, price_exponent, confidence)?;
        msg!("Updated prices");

        Ok(())
    }
}
