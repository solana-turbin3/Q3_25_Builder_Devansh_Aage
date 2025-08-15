use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Renter {
    pub score: i16,                   // 2 bytes - Credit score (0–1000 range)
    pub total_payments: u32,          // 4 bytes - Count of successful payments
    pub late_payments: u32,           // 4 bytes - Count of late payments
    pub bump: u8,
}