use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Renter {
    pub score: u16,                   // 2 bytes - Credit score (0–1000 range)
    pub total_payments: u32,          // 4 bytes - Count of successful payments
    pub late_payments: u32,           // 4 bytes - Count of late payments
    #[max_len(100)]
    pub past_agreements: Vec<Pubkey>, // Variable size - List of Agreement PDAs
    pub bump: u8,
}