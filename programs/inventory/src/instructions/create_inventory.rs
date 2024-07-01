use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, Program, Signer, System};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateInventory<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = signer
    )]
    pub merchant_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [Inventory::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + AssetInfo::SPACE,
        seeds = [AssetInfo::SEED_PREFIX.as_bytes(), asset_mint.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,

    #[account(
        init_if_needed,
        seeds = [main_const::VAULT, asset_mint.key().as_ref()],
        bump,
        payer = signer,
        token::mint = asset_mint,
        token::authority = asset_vault
    )]
    pub asset_vault: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_inventory(ctx: Context<CreateInventory>) -> Result<()> {
    //create asset_info
    ctx.accounts.asset_info.set_inner(AssetInfo::new(
        ctx.accounts.asset_mint.key(),
        ctx.accounts.merchant_usdc_account.key(),
    ));

    //add asset to inventory list
    ctx.accounts.inventory.insert_asset(
        &ctx.accounts.asset_mint,
        &ctx.accounts.signer,
        &ctx.accounts.system_program,
    )?;
    Ok(())
}
