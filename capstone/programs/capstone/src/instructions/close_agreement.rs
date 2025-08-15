use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

use crate::{Agreement, Renter};

#[derive(Accounts)]
pub struct CloseAgreement<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"deposit",agreement.key().as_ref()],
        bump=agreement.deposit_bump
    )]
    pub deposit_vault: SystemAccount<'info>,

    #[account(mut,
    close=landlord
)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub renter: Account<'info, Renter>,

    #[account(mut)]
    pub landlord: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer=signer,
         associated_token::mint=edition_mint,
        associated_token::token_program=token_program,
        associated_token::authority=landlord
    )]
    pub landlord_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program=token_program,
    )]
    pub edition_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint=edition_mint,
        associated_token::authority=agreement,
        associated_token::token_program=token_program
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseAgreement<'info> {
    pub fn close_agreement(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"agreement",
            self.agreement.renter.as_ref(),
            self.agreement.landlord.as_ref(),
            &[self.agreement.bump],
        ]];
        let transfer_nft_accounts = TransferChecked {
            authority: self.agreement.to_account_info(),
            from: self.nft_vault.to_account_info(),
            mint: self.edition_mint.to_account_info(),
            to: self.landlord_ata.to_account_info(),
        };
        let transfer_nft_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_nft_accounts,
            signer_seeds,
        );
        transfer_checked(transfer_nft_cpi, 1, self.edition_mint.decimals)?;

        let close_accounts = CloseAccount {
            account: self.nft_vault.to_account_info(),
            authority: self.agreement.to_account_info(),
            destination: self.landlord.to_account_info(),
        };
        let close_acc_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_acc_cpi)?;
        Ok(())
    }
}
