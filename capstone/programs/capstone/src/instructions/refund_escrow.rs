use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{burn_nft, BurnNft, Metadata},
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub landlord: Signer<'info>,

    #[account(
        mut,
        close=landlord,
        seeds=[b"escrow",edition_mint.key().as_ref()],
        bump=escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        mint::token_program=token_program,
        seeds=[b"edition",collection_mint.key().as_ref()],
        bump=escrow.edition_mint_bump
    )]
    pub edition_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),edition_mint.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(), edition_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program=token_metadata_program.key() //this tells anchor to derive this pda using token metadata program and not my program key which is default 
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint=edition_mint,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        mint::token_program=token_program,
        mint::decimals=0,
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub collection_metadata: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> Refund<'info> {
    pub fn burn_nft_and_close_vault(&mut self) -> Result<()> {
        let edition_mint_key = self.edition_mint.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"escrow", edition_mint_key.as_ref(), &[self.escrow.bump]]];

        let burn_nft_accounts = BurnNft {
            edition: self.master_edition.to_account_info(),
            metadata: self.metadata.to_account_info(),
            mint: self.edition_mint.to_account_info(),
            owner: self.escrow.to_account_info(),
            token: self.vault.to_account_info(),
            spl_token: self.token_program.to_account_info(),
        };

        let burn_nft_cpi = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            burn_nft_accounts,
            signer_seeds,
        );

        burn_nft(burn_nft_cpi, Some(*self.collection_metadata.key))?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.landlord.to_account_info(),
        };
        let close_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_cpi)?;
        Ok(())
    }
}
