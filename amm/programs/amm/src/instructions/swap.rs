use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds=[b"lp",config.key().as_ref()],
        bump=config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

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
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=user
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=mint_y,
        associated_token::authority=user
    )]
    pub user_y: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, amount_in: u64, is_x_to_y: bool, min_out: u64) -> Result<()> {
        require!(amount_in > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::PoolLocked);

        let mut curve: ConstantProduct = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        let liquidity_pair = match is_x_to_y {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let swap_result = curve
            .swap(liquidity_pair, amount_in, min_out)
            .map_err(AmmError::from)?;

        require!(swap_result.deposit != 0, AmmError::InvalidAmount);
        require!(swap_result.withdraw != 0, AmmError::InvalidAmount);

        self.deposit(swap_result.deposit, is_x_to_y)?;
        self.withdraw(swap_result.withdraw, !is_x_to_y)?;

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64, is_x: bool) -> Result<()> {
        require!(amount > 0, AmmError::InvalidAmount);

        let (from, to) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };

        let cpi_accounts = Transfer {
            authority: self.user.to_account_info(),
            from,
            to,
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64, is_x: bool) -> Result<()> {
        require!(amount > 0, AmmError::InvalidAmount);

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

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config".as_ref(),
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ]];

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
}
