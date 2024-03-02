use crate::state::{AssetInfo, Inventory};
use crate::utils::InventoryError;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

pub fn buy_asset(ctx: Context<BuyAsset>, amount: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    //check if there's enough
    if amount > asset_info.amount {
        return Err(InventoryError::InsufficientInventoryAsset.into());
    }
    //calculate cost
    let cost = amount * asset_info.price;

    //check if user has enough balance
    if cost > ctx.accounts.payer_usdc_account.amount {
        return Err(InventoryError::InsufficientUSDC.into());
    }

    //transfer from payer to p.developer
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_usdc_account.to_account_info(),
                to: ctx.accounts.dev_usdc_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        cost,
    )?;
    //transfer token to payer
    // deduct amount of asset purchased
    asset_info.amount -= amount;
    Ok(())
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

    pub mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
