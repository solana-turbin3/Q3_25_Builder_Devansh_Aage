use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorCode, Agreement, Renter};

#[derive(Accounts)]
pub struct MonthlyRent<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub landlord: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[b"deposit",agreement.key().as_ref()],
        bump=agreement.deposit_bump
    )]
    pub deposit_vault: SystemAccount<'info>,

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,
    #[account(mut)]
    pub renter: Account<'info, Renter>,

    pub system_program: Program<'info, System>,
}

impl<'info> MonthlyRent<'info> {
    pub fn monthly_rent(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        let seconds_in_month: i64 = 30 * 24 * 60 * 60; // 2_592_000 seconds
        let month_offset = i64::from(self.agreement.payments_made)
            .checked_add(1)
            .and_then(|v| v.checked_mul(seconds_in_month))
            .ok_or(ErrorCode::Overflow)?;
        let rent_due_date = self
            .agreement
            .start_date
            .checked_add(month_offset)
            .ok_or(ErrorCode::Overflow)?;

        if now > rent_due_date {
            self.pay_from_deposit()?;
            self.add_record_and_decrement_score()?;
        } else {
            self.pay_rent()?;
            self.add_record_and_increment_score()?;
        }
        Ok(())
    }
    pub fn pay_rent(&mut self) -> Result<()> {
        let payment_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.landlord.to_account_info(),
        };

        let pay_rent_cpi = CpiContext::new(self.system_program.to_account_info(), payment_accounts);

        transfer(pay_rent_cpi, self.agreement.rent_amount)?;
        Ok(())
    }

    pub fn pay_from_deposit(&mut self) -> Result<()> {
        let agreement_key = self.agreement.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"deposit",
            agreement_key.as_ref(),
            &[self.agreement.deposit_bump],
        ]];
        let transfer_accounts = Transfer {
            from: self.deposit_vault.to_account_info(),
            to: self.landlord.to_account_info(),
        };
        let transfer_cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );
        let late_rent_fine_fee = self
            .agreement
            .rent_amount
            .checked_mul(u64::from(self.agreement.late_fee_percent))
            .and_then(|v| v.checked_div(100))
            .ok_or(ErrorCode::Overflow)?;
        let total_rent = self
            .agreement
            .rent_amount
            .checked_add(late_rent_fine_fee)
            .ok_or(ErrorCode::Overflow)?;
        transfer(transfer_cpi, total_rent)?;

        Ok(())
    }

    pub fn add_record_and_decrement_score(&mut self) -> Result<()> {
        self.agreement.payments_made = self
            .agreement
            .payments_made
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        self.renter.score = self
            .renter
            .score
            .checked_sub(2)
            .ok_or(ErrorCode::Overflow)?;
        self.renter.total_payments = self
            .renter
            .total_payments
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        self.renter.late_payments = self
            .renter
            .late_payments
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    pub fn add_record_and_increment_score(&mut self) -> Result<()> {
        self.agreement.payments_made = self
            .agreement
            .payments_made
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        self.renter.score = self
            .renter
            .score
            .checked_add(2)
            .ok_or(ErrorCode::Overflow)?;
        self.renter.total_payments = self
            .renter
            .total_payments
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}
