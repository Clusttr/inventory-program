use crate::state::AssetInfo;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct UpdateAssetInfo<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = [AssetInfo::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
    bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn update_asset_info(ctx: Context<UpdateAssetInfo>, new_price: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.price = new_price;
    Ok(())
}
