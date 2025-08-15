use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Bits Overflowed!")]
    Overflow,
    #[msg("Unauthorized!")]
    Unauthorized,
}
