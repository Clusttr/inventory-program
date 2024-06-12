use std::str::FromStr;
use crate::state::{AssetInfo, AssetInfoAccount, Inventory};
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    require_keys_eq!(ctx.accounts.usdc_mint.key(), Pubkey::from_str(main_const::USDC).unwrap());

    let deposit = (
        &ctx.accounts.buyer_usdc_account,
        &ctx.accounts.dev_usdc_account,
    );
    let receive = (
        &ctx.accounts.asset_mint,
        &ctx.accounts.mint_vault,
        &ctx.accounts.buyer_asset_account,
        ctx.bumps.mint_vault,
    );

    ctx.accounts.asset_info.buy(
        deposit,
        receive,
        amount,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )
}

#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority = signer,
    )]
    pub buyer_usdc_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    associated_token::mint = asset_mint,
    associated_token::authority = signer,
    )]
    pub buyer_asset_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority = signer,
    )]
    pub dev_usdc_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [Inventory::SEED_PREFIX.as_bytes()],
    bump,
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(
    mut,
    seeds = [AssetInfo::SEED_PREFIX.as_bytes(), asset_mint.key().as_ref()],
    bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,

    #[account(
    mut,
    seeds = [main_const::VAULT, asset_mint.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub mint_vault: Account<'info, TokenAccount>,
    pub asset_mint: Account<'info, Mint>,

    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
