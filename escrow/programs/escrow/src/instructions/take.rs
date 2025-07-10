use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::Escrow;

use crate::error::ErrorCode::InvalidAmount;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_b,
        associated_token::token_program=token_program,
        associated_token::authority=maker
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::token_program=token_program,
        associated_token::authority=taker
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close=taker,
        has_one=maker,
        has_one=mint_a,
        seeds=[b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::token_program=token_program,
        associated_token::mint=mint_a,
        associated_token::authority=escrow
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn transfer_to_maker(&mut self, transfer: u64) -> Result<()> {
        if transfer != self.escrow.receive {
            return Err(InvalidAmount.into());
        }
        let tx_cpi_accounts = TransferChecked {
            authority: self.taker.to_account_info(),
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let tx_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), tx_cpi_accounts);

        transfer_checked(tx_cpi_ctx, transfer, self.mint_b.decimals)?;

        msg!("Transferred to maker!");
        Ok(())
    }

    pub fn transfer_to_taker(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
        let tx_cpi_accounts = TransferChecked {
            authority: self.escrow.to_account_info(),
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
        };

        let tx_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            tx_cpi_accounts,
            &signer_seeds,
        );

        transfer_checked(tx_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        msg!("Transferred to Taker!");

        Ok(())
    }

    pub fn close_escrow(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &signer_seeds,
        );

        close_account(cpi_ctx)?;

        msg!("Escrow Closed!");

        Ok(())
    }
}
