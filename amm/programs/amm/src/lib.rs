pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DT8STwJ1N5cbRa4TRhD6TnNMKCySFd3exupZtJH39J1K");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_amm(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Pubkey,
    ) -> Result<()> {
        let _ = ctx.accounts.init(seed, fee, Some(authority), ctx.bumps);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        // amount is the number of lp tokens user wants
        // max_x is the number of x type tokens user is willing to deposit
        // max_y is the number of y type tokens user is willing to deposit

        let _ = ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, is_x_to_y: bool, min_out: u64) -> Result<()> {
        let _ = ctx.accounts.swap(amount_in, is_x_to_y, min_out)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount_lp: u64, min_x: u64, min_y: u64) -> Result<()> {
        let _ = ctx.accounts.withdraw(amount_lp, min_x, min_y)?;
        Ok(())
    }
}
