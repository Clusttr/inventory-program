use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub fn add_asset(ctx: Context<AddAsset>) -> Result<()> {
    ctx.accounts.inventory.insert_asset(
        &ctx.accounts.mint,
        &ctx.accounts.payer,
        &ctx.accounts.system_program,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct AddAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [Inventory::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub inventory: Account<'info, Inventory>,

    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}
