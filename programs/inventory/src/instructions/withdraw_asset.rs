use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, transfer, Transfer};

#[derive(Accounts)]
pub struct WithdrawAsset<'info> {
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

pub fn withdraw_asset(ctx: Context<WithdrawAsset>, amount: u64) -> Result<()> {
    // confirm vault has enough asset
    require!(ctx.accounts.asset_vault.amount > amount, InventoryError::InsufficientAsset);

    let mint_key = ctx.accounts.asset_mint.key();
    let seed: &[&[&[u8]]] = &[&[main_const::VAULT, mint_key.as_ref(), &[ctx.bumps.asset_vault]]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.asset_vault.to_account_info(),
                to: ctx.accounts.user_asset_account.to_account_info(),
                authority: ctx.accounts.asset_vault.to_account_info(),
            },
            seed,
        ),
        amount,
    )?;
    Ok(())
}
