use crate::state::{AssetInfo, Inventory, InventoryAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, CloseAccount, Mint, Token, TokenAccount};
use crate::utils::*;

#[derive(Accounts)]
pub struct CloseInventory<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [Inventory::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub inventory: Account<'info, Inventory>,

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
        close = signer
    )]
    pub asset_info: Account<'info, AssetInfo>,
    pub asset_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn close_inventory(ctx: Context<CloseInventory>) -> Result<()> {
    require!(ctx.accounts.asset_vault.amount == 0, InventoryError::InventoryVaultNotEmpty);

    let mint_key = &ctx.accounts.asset_mint.key();
    let asset_vault_seed: &[&[&[u8]]] = &[&[main_const::VAULT, mint_key.as_ref(), &[ctx.bumps.asset_vault]]];

    //close asset vault
    close_token_account(&ctx.accounts.asset_vault,
                        &ctx.accounts.signer,
                        &ctx.accounts.token_program,
                        asset_vault_seed)?;

    ctx.accounts.inventory.remove_asset(&ctx.accounts.asset_mint)?;
    Ok(())
}

fn close_token_account<'info>(account: &Account<'info, TokenAccount>,
                              destination: &AccountInfo<'info>,
                              token_program: &Program<'info, Token>,
                              seed: &[&[&[u8]]]) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: account.to_account_info(),
        destination: destination.to_account_info(),
        authority: account.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, seed);
    close_account(cpi_ctx)
}

