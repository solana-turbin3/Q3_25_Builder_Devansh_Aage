use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{error::ErrorCode, state::Agreement, Escrow};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub renter: Signer<'info>,

    #[account(mut)]
    pub landlord: SystemAccount<'info>,

    #[account(
        init,
        payer=renter,
        space=8+Agreement::INIT_SPACE,
        seeds=[b"agreement",renter.key().as_ref(),landlord.key().as_ref()],
        bump
    )]
    pub agreement: Account<'info, Agreement>,

    #[account(
        mut,
        close=landlord,
        seeds=[b"escrow",edition_mint.key().as_ref()],
        bump=escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mint::token_program=token_program,
    )]
    pub edition_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=edition_mint,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"deposit",agreement.key().as_ref()],
        bump
    )]
    pub deposit_vault: SystemAccount<'info>,

    #[account(
        init,
        payer=renter,
        associated_token::mint=edition_mint,
        associated_token::authority=agreement,
        associated_token::token_program=token_program
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Take<'info> {
    pub fn init_agreement_pda(&mut self, bumps: &TakeBumps) -> Result<()> {
        let start_date = Clock::get()?.unix_timestamp;

        // Approximate months in seconds (30 days each)
        let seconds_in_month = 30 * 24 * 60 * 60; // 2_592_000 seconds
        let total_seconds = i64::from(self.escrow.months)
            .checked_mul(seconds_in_month)
            .ok_or(ErrorCode::Overflow)? as i64;
        let end_date = start_date
            .checked_add(total_seconds)
            .ok_or(ErrorCode::Overflow)?;

        self.agreement.set_inner(Agreement {
            landlord: *self.landlord.key,
            renter: *self.renter.key,
            start_date,
            end_date,
            rent_amount: self.escrow.monthly_rent,
            deposit_amount: self.escrow.deposit_amount,
            late_fee_percent: self.escrow.late_fee_percent,
            cancel_allowed_after: self.escrow.cancel_allowed_after,
            cancel_penalty_percent: self.escrow.cancel_penalty_percent,
            payments_made: 0,
            bump: bumps.agreement,
            deposit_bump: bumps.deposit_vault,
        });
        Ok(())
    }

    pub fn transfer_deposit(&mut self) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.renter.to_account_info(),
            to: self.deposit_vault.to_account_info(),
        };

        let transfer_cpi_ctx =
            CpiContext::new(self.system_program.to_account_info(), transfer_accounts);

        transfer(transfer_cpi_ctx, self.agreement.deposit_amount)?;
        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        let edition_mint_key = self.edition_mint.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"escrow", edition_mint_key.as_ref(), &[self.escrow.bump]]];

        let transfer_nft_accounts = TransferChecked {
            authority: self.escrow.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.nft_vault.to_account_info(),
            mint: self.edition_mint.to_account_info(),
        };
        let transfer_nft_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_nft_accounts,
            signer_seeds,
        );
        transfer_checked(transfer_nft_cpi_ctx, 1, self.edition_mint.decimals)?;
        Ok(())
    }
}
