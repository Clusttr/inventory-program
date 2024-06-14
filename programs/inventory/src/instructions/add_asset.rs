use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, transfer, Transfer};

#[derive(Accounts)]
pub struct AddAsset<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    mut,
    associated_token::mint = asset_mint,
    associated_token::authority = signer
    )]
    pub user_asset_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [main_const::VAULT, asset_mint.key().as_ref()],
    bump,
    token::mint = asset_mint,
    token::authority = asset_vault
    )]
    pub asset_vault: Account<'info, TokenAccount>,
    pub asset_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn add_asset(ctx: Context<AddAsset>, amount: u64) -> Result<()> {
    if ctx.accounts.user_asset_account.amount < amount {
        return Err(InventoryError::InsufficientAsset.into());
    }

    //transfer asset to vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_asset_account.to_account_info(),
                to: ctx.accounts.asset_vault.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
