mod instructions;
mod state;
mod utils;

use crate::state::Inventory;
use anchor_lang::prelude::*;
use instructions::*;

declare_id!("8QW6oBt7NvXN68Cy7yjKUGa6rFPi5EtMmXrp8hoUmwpw");

#[program]
pub mod inventory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.inventory.set_inner(Inventory::new());
        Ok(())
    }

    pub fn create_inventory(ctx: Context<CreateInventory>, price: u64) -> Result<()> {
        instructions::create_inventory(ctx, price)
    }

    pub fn close_inventory(ctx: Context<CloseInventory>) -> Result<()> {
        instructions::close_inventory(ctx)
    }

    pub fn add_asset(ctx: Context<AddAsset>) -> Result<()> {
        instructions::add_asset(ctx)
    }

    pub fn withdraw_asset(ctx: Context<WithdrawAsset>) -> Result<()> {
        instructions::withdraw_asset(ctx)
    }

    pub fn update_asset_info(
        ctx: Context<UpdateAssetInfo>,
        new_price: u64,
        new_usdc_account: Pubkey,
    ) -> Result<()> {
        instructions::update_asset_info(ctx, new_price, new_usdc_account)
    }

    pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
        instructions::buy_asset(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    payer = payer,
    space = Inventory::SPACE,
    seeds = [Inventory::SEED_PREFIX.as_bytes()],
    bump,
    )]
    pub inventory: Account<'info, Inventory>,
    pub system_program: Program<'info, System>,
}
