use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Agreement {
    pub landlord: Pubkey,           // 32 bytes (i think dont need this)
    pub renter: Pubkey,             // 32 bytes (i think dont need this)
    pub start_date: i64,            // 8 bytes - Unix timestamp (seconds)
    pub end_date: i64,              // 8 bytes
    pub rent_amount: u64,           // 8 bytes
    pub deposit_amount: u64,        // 8 bytes
    pub status: u8,                 // 1 byte (0=Active,1=Closed,2=Defaulted)
    pub late_fee_percent: u8,       // 1 byte
    pub cancel_allowed_after: u16,  // 2 bytes
    pub cancel_penalty_percent: u8, // 1 byte
    pub payments_made: u16,         // 2 bytes - Number of successful payments
    pub bump: u8,
}

#[repr(u8)]
pub enum AgreementStatus {
    Active = 0,
    Closed = 1,
    Defaulted = 2,
}
