pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3ECCL8btDKSnNYEgu15UZca4epL2PdHqDEYdY9UBvmcP");

#[program]
pub mod capstone {
    use super::*;

    pub fn init_landlord(
        ctx: Context<InitLandlord>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let _ = ctx.accounts.mint_master_edition_nft(name, symbol, uri);
        Ok(())
    }

    pub fn init_renter(ctx: Context<InitRenter>) -> Result<()> {
        ctx.accounts.init_renter(&ctx.bumps)?;
        msg!("Init Renter");
        Ok(())
    }

    pub fn create_escrow(
        ctx: Context<MakeEscrow>,
        monthly_rent: u64,
        deposit_amount: u64,
        late_fee_percent: u8,
        min_renter_score: u16,
        cancel_allowed_after: u16,
        cancel_penalty_percent: u8,
        months: u8,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        ctx.accounts.init_escrow(
            &ctx.bumps,
            monthly_rent,
            deposit_amount,
            late_fee_percent,
            min_renter_score,
            cancel_allowed_after,
            cancel_penalty_percent,
            months,
        )?;
        msg!("Init Escrow PDA!");
        ctx.accounts
            .mint_edition_nft(&ctx.bumps, nft_name, nft_symbol, nft_uri)?;
        msg!("Minted Printable NFT PDA!");
        Ok(())
    }

    pub fn close_escrow(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.burn_nft_and_close_vault()?;
        msg!("Close Escrow");
        Ok(())
    }

    pub fn take_escrow(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.init_agreement_pda(&ctx.bumps)?;
        msg!("Init Agreement PDA");
        ctx.accounts.transfer_deposit()?;
        msg!("Transfer Deposit");
        ctx.accounts.transfer_nft()?;
        msg!("Transfer NFT");
        Ok(())
    }
}
