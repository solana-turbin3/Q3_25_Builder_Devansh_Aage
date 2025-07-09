use anchor_lang::prelude::*;
use anchor_spl::{   token_interface::{mint_to,MintTo,TokenInterface,TokenAccount,Mint}};
use crate::Escrow;
#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}


