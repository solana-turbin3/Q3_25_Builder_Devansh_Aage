pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3ECCL8btDKSnNYEgu15UZca4epL2PdHqDEYdY9UBvmcP");

#[program]
pub mod capstone {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }

    pub fn init_landlord(
        ctx: Context<InitLandlord>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let _ = ctx.accounts.mint_master_edition_nft(name, symbol, uri);
        Ok(())
    }
}
