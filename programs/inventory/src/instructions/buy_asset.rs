use std::str::FromStr;
use crate::state::{Asset, AssetInfo, AssetInfoAccount};
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount};

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

    #[account()]
    pub merchant_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [main_const::VAULT, asset_mint.key().as_ref()],
        bump,
    )]
    pub asset_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [AssetInfo::SEED_PREFIX.as_bytes(), asset_mint.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    /// CHECK: will add verification later
    // #[account(
    //     seeds = [asset_mint.key().as_ref()],
    //     bump
    // )]
    pub asset_price: AccountInfo<'info>,

    pub asset_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    /// CHECK: this is the program address for clusttr price oracle
    #[account(
        address = Pubkey::from_str(main_const::PRICE_ORACLE).unwrap()
    )]
    pub price_oracle: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    require_keys_eq!(ctx.accounts.usdc_mint.key(), Pubkey::from_str(main_const::USDC).unwrap());
    // let (_pda, _) = Pubkey::find_program_address(&[ctx.accounts.asset_mint.key().as_ref()], &ctx.accounts.price_oracle.key());
    let asset = Asset::from_account_info(&ctx.accounts.asset_price);

    let deposit = (
        &ctx.accounts.buyer_usdc_account,
        &ctx.accounts.merchant_usdc_account,
    );
    let receive = (
        &ctx.accounts.asset_mint,
        &ctx.accounts.asset_vault,
        &ctx.accounts.buyer_asset_account,
        ctx.bumps.asset_vault,
    );

    ctx.accounts.asset_info.buy(
        deposit,
        receive,
        asset.value,
        amount,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )
}
