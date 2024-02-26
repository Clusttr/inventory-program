mod instructions;
mod state;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Dy4hgMge7QbYHZiNLzgBoJg155h2zfLPafdRMJVxWfY3");

#[program]
pub mod inventory {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_asset(ctx: Context<AddAsset>) -> Result<()> {
        instructions::add_asset(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
