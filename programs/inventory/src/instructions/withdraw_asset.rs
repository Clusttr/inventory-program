use crate::state::{Inventory, InventoryAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub fn withdraw_asset(ctx: Context<WithdrawAsset>) -> Result<()> {
    ctx.accounts.inventory.remove_asset(&ctx.accounts.mint)
}

#[derive(Accounts)]
pub struct WithdrawAsset<'info> {
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
