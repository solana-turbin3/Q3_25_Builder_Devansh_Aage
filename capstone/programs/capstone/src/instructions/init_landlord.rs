use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
        sign_metadata, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, SignMetadata,
    },
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct InitLandlord<'info> {
    #[account(mut)]
    pub landlord: Signer<'info>,

    #[account(
        init,
        payer=landlord,
        mint::decimals=0,
        mint::authority=landlord,
        mint::freeze_authority=landlord,
        seeds=[b"collection_mint".as_ref(),landlord.key().as_ref()],
        bump,
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=landlord,
        associated_token::authority=landlord,
        associated_token::mint=collection_mint,
        associated_token::token_program=token_program,
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account is checked by the metadata program
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(), collection_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program=token_metadata_program.key() //this tells anchor to derive this pda using token metadata program and not my program key which is default 
    )]
    pub master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitLandlord<'info> {
    pub fn mint_master_edition_nft(
        &mut self,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Creating Mint account!");

        let mint_cpi_accounts = MintTo {
            authority: self.landlord.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            to: self.collection_token_account.to_account_info(),
        };

        let mint_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), mint_cpi_accounts);

        mint_to(mint_cpi_ctx, 1)?;

        msg!("Creating Metadata Account");

        let metadata_cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            mint_authority: self.landlord.to_account_info(),
            payer: self.landlord.to_account_info(),
            rent: self.rent.to_account_info(),
            update_authority: self.landlord.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: self.landlord.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        let metadata_cpi_ctx = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            metadata_cpi_accounts,
        );

        create_metadata_accounts_v3(
            metadata_cpi_ctx,
            data_v2,
            true,
            true,
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        msg!("Creating Master Edition account");

        let master_edition_cpi_accounts = CreateMasterEditionV3 {
            payer: self.landlord.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            edition: self.master_edition.to_account_info(),
            mint_authority: self.landlord.to_account_info(),
            update_authority: self.landlord.to_account_info(),
            metadata: self.metadata.to_account_info(),
            rent: self.rent.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let master_edition_cpi_ctx = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            master_edition_cpi_accounts,
        );
        create_master_edition_v3(master_edition_cpi_ctx, Some(0))?;

        msg!("Verifying the Collection");

        let sign_metadata_cpi_accounts = SignMetadata {
            creator: self.landlord.to_account_info(),
            metadata: self.metadata.to_account_info(),
        };

        let sign_metadata_cpi_ctx = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            sign_metadata_cpi_accounts,
        );

        sign_metadata(sign_metadata_cpi_ctx)?;
        Ok(())
    }
}
