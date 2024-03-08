use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn add_asset(ctx: Context<AddAsset>, amount: u64) -> Result<()> {
    let deposit = (
        &ctx.accounts.mint,
        &ctx.accounts.user_asset_account,
        &ctx.accounts.mint_vault,
        amount,
    );
    ctx.accounts.asset_info.add(
        deposit,
        &ctx.accounts.payer,
        &mut ctx.accounts.inventory,
        &ctx.accounts.token_program,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct AddAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub user_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [Inventory::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(
    mut,
    seeds = [AssetInfo::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
    bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,

    #[account(
    init_if_needed,
    seeds = [main_const::VAULT, mint.key().as_ref()],
    bump,
    payer = payer,
    token::mint = mint,
    token::authority = payer
    )]
    pub mint_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
