use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, Program, Signer, System};
use anchor_spl::token::Mint;

pub fn create_inventory(ctx: Context<CreateInventory>) -> Result<()> {
    ctx.accounts.inventory.insert_asset(
        &ctx.accounts.mint,
        &ctx.accounts.payer,
        &ctx.accounts.system_program,
    )?;
    Ok(())
}
#[derive(Accounts)]
pub struct CreateInventory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [Inventory::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(
        init,
        payer = payer,
        space = 8 + AssetInfo::SPACE,
        seeds = [AssetInfo::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}
