use anchor_lang::prelude::*;

use crate::Renter;

#[derive(Accounts)]
pub struct InitRenter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space=8+Renter::INIT_SPACE,
        payer=signer,
        seeds=[b"renter",signer.key().as_ref()],
        bump
    )]
    pub renter: Account<'info, Renter>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitRenter<'info> {
    pub fn init_renter(&mut self, bumps: &InitRenterBumps) -> Result<()> {
        self.renter.set_inner(Renter {
            score: 20,
            total_payments: 0,
            late_payments: 0,
            bump: bumps.renter,
        });
        Ok(())
    }
}
