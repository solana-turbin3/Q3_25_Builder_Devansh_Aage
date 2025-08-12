use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Agreement {
    pub landlord: Pubkey,           // 32 bytes
    pub renter: Pubkey, // 32 bytes we need so we can use renter for fn like close agreement we can check if its right account calling it
    pub start_date: i64, // 8 bytes - Unix timestamp (seconds)
    pub end_date: i64,  // 8 bytes
    pub rent_amount: u64, // 8 bytes in lamports
    pub deposit_amount: u64, // 8 bytes in lamports
    pub late_fee_percent: u8, // 1 byte
    pub cancel_allowed_after: u16, // 2 bytes
    pub cancel_penalty_percent: u8, // 1 byte
    pub payments_made: u16, // 2 bytes - Number of successful payments
    pub bump: u8,
    pub deposit_bump: u8,
}
