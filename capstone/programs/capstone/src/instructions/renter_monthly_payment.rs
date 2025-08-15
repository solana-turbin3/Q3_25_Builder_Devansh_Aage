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

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,
    #[account(mut)]
    pub renter: Account<'info, Renter>,

    pub system_program: Program<'info, System>,
}

impl<'info> MonthlyRent<'info> {
    pub fn pay_rent(&mut self) -> Result<()> {
        let payment_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.landlord.to_account_info(),
        };

        let pay_rent_cpi = CpiContext::new(self.system_program.to_account_info(), payment_accounts);

        transfer(pay_rent_cpi, self.agreement.rent_amount)?;
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
