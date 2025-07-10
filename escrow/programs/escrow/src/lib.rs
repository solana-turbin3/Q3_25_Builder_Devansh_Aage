pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5wenfcksfPtLhzbt5vWZS5FQg6xJyKm8GgWWH3993uaA");

#[program]
pub mod escrow {
    use super::*;

    pub fn init_escrow(ctx: Context<Make>, seed: u64, receive: u64) -> Result<()> {
        let _ = ctx.accounts.init_escrow(seed, receive, &ctx.bumps);
        Ok(())
    }

    pub fn deposit(ctx: Context<Make>, deposit: u64) -> Result<()> {
        let _ = ctx.accounts.deposit(deposit);
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>, receive:u64) -> Result<()> {
        ctx.accounts.transfer_to_maker(receive)?;
        ctx.accounts.transfer_to_taker()?;
        ctx.accounts.close_escrow()?;
        Ok(())
    }
}
