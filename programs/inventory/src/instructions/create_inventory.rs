use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, Program, Signer, System};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn create_inventory(ctx: Context<CreateInventory>, price: u64) -> Result<()> {
    let nft_mint = ctx.accounts.mint.key();
    let usdc_account = ctx.accounts.usdc_account.key();
    ctx.accounts
        .asset_info
        .set_inner(AssetInfo::new(nft_mint, price, usdc_account));
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

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = usdc_mint,
        associated_token::authority = payer
    )]
    pub usdc_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
