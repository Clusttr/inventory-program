use crate::state::{AssetInfo, AssetInfoAccount, Inventory};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    let deposit = (
        &ctx.accounts.usdc_mint,
        &ctx.accounts.payer_usdc_account,
        &ctx.accounts.dev_usdc_account,
    );
    let receive = (
        &ctx.accounts.mint,
        &ctx.accounts.mint_vault,
        &ctx.accounts.payer_mint_account,
    );

    ctx.accounts.asset_info.buy(
        deposit,
        receive,
        amount,
        &ctx.accounts.payer,
        &ctx.accounts.token_program,
    )
}
#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = payer,
    )]
    pub payer_usdc_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = payer,
    )]
    pub payer_mint_account: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = usdc_mint,
        associated_token::authority = payer,
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
    seeds = [AssetInfo::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
    bump,
    close = payer
    )]
    pub asset_info: Account<'info, AssetInfo>,

    #[account(
    mut,
    token::mint = mint,
    token::authority = mint_vault
    )]
    pub mint_vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
