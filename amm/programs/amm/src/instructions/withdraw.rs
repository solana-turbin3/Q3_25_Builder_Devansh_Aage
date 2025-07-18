use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(mut,
    associated_token::mint=mint_x,
    associated_token::authority=user)]
    pub user_x: Account<'info, TokenAccount>,
    #[account(mut,
    associated_token::mint=mint_y,
    associated_token::authority=user)]
    pub user_y: Account<'info, TokenAccount>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump=config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(mut,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(mut,
        associated_token::mint=mint_y,
        associated_token::authority=config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=mint_lp,
        associated_token::authority=user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount_lp: u64, min_x: u64, min_y: u64) -> Result<()> {
        require!(amount_lp > 0, AmmError::InvalidAmount);
        require!(self.config.locked == false, AmmError::PoolLocked);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount_lp,
            6,
        )
        .map_err(AmmError::from)?;

        require!(amounts.x >= min_x, AmmError::SlippageExceeded);
        require!(amounts.y >= min_y, AmmError::SlippageExceeded);

        self.withdraw_tokens(amounts.x, true)?;
        self.withdraw_tokens(amounts.y, false)?;

        self.burn_lp(amount_lp)?;

        Ok(())
    }

    pub fn withdraw_tokens(&mut self, amount: u64, is_x: bool) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        let (from, to) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x.to_account_info(),
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y.to_account_info(),
            ),
        };

        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_be_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            authority: self.config.to_account_info(),
            from,
            to,
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn burn_lp(&mut self, amount: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        let cpi_accounts = Burn {
            authority: self.user.to_account_info(),
            from: self.user_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        burn(cpi_ctx, amount)?;
        Ok(())
    }
}
