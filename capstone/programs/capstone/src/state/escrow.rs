use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub landlord: Pubkey, // 32 bytes - Wallet address of landlord (i think dont need this)
    pub monthly_rent: u64, // 8 bytes - Rent amount in smallest token unit (e.g., USDC 6 decimals)
    pub deposit_amount: u64, // 8 bytes - Security deposit
    pub late_fee_percent: u8, // 1 byte - % late fee
    pub min_renter_score: u16, // 2 bytes - Required renter score
    pub cancel_allowed_after: u16, // 2 bytes - In months
    pub cancel_penalty_percent: u8, // 1 byte - % penalty
    pub edition_mint: Pubkey, // 32 bytes - PDA token account to store edition NFT (i think dont need this)
    pub bump: u8,
}
