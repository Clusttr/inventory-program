use crate::state::Inventory;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.inventory.set_inner(Inventory::new());
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init,
    payer = signer,
    space = Inventory::SPACE,
    seeds = [Inventory::SEED_PREFIX.as_bytes()],
    bump,
    )]
    pub inventory: Account<'info, Inventory>,
    pub system_program: Program<'info, System>,
}
