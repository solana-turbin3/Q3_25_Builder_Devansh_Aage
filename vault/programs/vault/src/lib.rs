use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("2BQQ86qu7iRZX9bnqGAi3ZGdpk8ZCxgdjj7rrbcgPgXq");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let _ = ctx.accounts.initialize(&ctx.bumps);
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let _ = ctx.accounts.deposit(amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let _ = ctx.accounts.withdraw(amount);
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let _ = ctx.accounts.close();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init,
    payer=user,
    seeds=[b"state",user.key().as_ref()],
    bump,
    space= VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds=[b"vault",user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let rent_exempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, rent_exempt)?;

        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
         seeds=[b"vault",user.key().as_ref()],
        bump=vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
    seeds=[b"state",user.key().as_ref()],
    bump=vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    fn withdraw(&mut self, amount: u64) -> Result<()> {
        let rent_lamports = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let balance = self.vault.get_lamports();
        if rent_lamports > balance {
            return Err(ErrorCode::BalanceTooLow.into());
        }
        let amount_withdrawable = balance - rent_lamports;
        if amount > amount_withdrawable {
            return Err(ErrorCode::InsufficientBalance.into());
        }
        let seeds = &[
            b"vault",
            self.user.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
         seeds=[b"vault",user.key().as_ref()],
        bump=vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
    mut,
    close=user,
    seeds=[b"state",user.key().as_ref()],
    bump=vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds = &[
            b"vault",
            self.user.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let balance = self.vault.get_lamports();
        transfer(cpi_ctx, balance)?;
        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 * 2;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not Sufficient Balance!")]
    InsufficientBalance,
    #[msg("Balance less than rent exempt!")]
    BalanceTooLow,
}
