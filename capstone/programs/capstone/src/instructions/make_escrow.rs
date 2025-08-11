use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        set_and_verify_sized_collection_item, CreateMasterEditionV3, CreateMetadataAccountsV3,
        Metadata, SetAndVerifySizedCollectionItem,
    },
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
pub struct MakeEscrow<'info> {
    #[account(mut)]
    pub landlord: Signer<'info>,

    #[account(
        init,
        payer=landlord,
        space=8+Escrow::INIT_SPACE,
        seeds=[b"escrow",edition_mint.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer=landlord,
        mint::decimals=0,
        mint::authority=collection_mint,
        mint::freeze_authority=collection_mint,
        mint::token_program=token_program,
        seeds=[b"edition".as_ref(),collection_mint.key().as_ref()],
        bump
    )]
    pub edition_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=landlord,
        associated_token::mint=edition_mint,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

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
        seeds=[b"collection_mint".as_ref(),landlord.key().as_ref()],
        bump,
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

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(), collection_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program=token_metadata_program.key() //this tells anchor to derive this pda using token metadata program and not my program key which is default 
    )]
    pub collection_master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MakeEscrow<'info> {
    pub fn mint_edition_nft(
        &mut self,
        bumps: &MakeEscrowBumps,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        //To-Do add safety checks if an escrow or agreement already exists

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            self.landlord.key.as_ref(),
            &[bumps.collection_mint],
        ]];

        let mint_cpi_accounts = MintTo {
            authority: self.collection_mint.to_account_info(),
            mint: self.edition_mint.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let mint_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_cpi_accounts,
            &signer_seeds,
        );

        mint_to(mint_cpi_ctx, 1)?;

        let metadata_cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.edition_mint.to_account_info(),
            mint_authority: self.collection_mint.to_account_info(),
            payer: self.landlord.to_account_info(),
            rent: self.rent.to_account_info(),
            update_authority: self.collection_mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            metadata_cpi_accounts,
            &signer_seeds,
        );

        create_metadata_accounts_v3(metadata_cpi_ctx, data_v2, true, true, None)?;

        let master_edition_cpi_accounts = CreateMasterEditionV3 {
            payer: self.landlord.to_account_info(),
            mint: self.edition_mint.to_account_info(),
            edition: self.collection_master_edition.to_account_info(),
            mint_authority: self.landlord.to_account_info(),
            update_authority: self.landlord.to_account_info(),
            metadata: self.metadata.to_account_info(),
            rent: self.rent.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let master_edition_cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            master_edition_cpi_accounts,
            &signer_seeds,
        );
        create_master_edition_v3(master_edition_cpi_ctx, Some(0))?;

        let verify_sized_collection_accounts = SetAndVerifySizedCollectionItem {
            metadata: self.metadata.to_account_info(),
            collection_authority: self.landlord.to_account_info(),
            payer: self.landlord.to_account_info(),
            update_authority: self.landlord.to_account_info(),
            collection_metadata: self.collection_metadata.to_account_info(),
            collection_master_edition: self.collection_master_edition.to_account_info(),
            collection_mint: self.collection_mint.to_account_info(),
        };
        let verify_sized_collection_cpi = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            verify_sized_collection_accounts,
            &signer_seeds,
        );

        set_and_verify_sized_collection_item(verify_sized_collection_cpi, None)?;
        Ok(())
    }

    pub fn init_escrow(
        &mut self,
        bumps: &MakeEscrowBumps,
        monthly_rent: u64,
        deposit_amount: u64,
        late_fee_percent: u8,
        min_renter_score: u16,
        cancel_allowed_after: u16,
        cancel_penalty_percent: u8,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            landlord: *self.landlord.key,
            monthly_rent,
            deposit_amount,
            late_fee_percent,
            min_renter_score,
            cancel_allowed_after,
            cancel_penalty_percent,
            bump: bumps.escrow,
            edition_mint_bump: bumps.edition_mint,
        });
        Ok(())
    }
}
