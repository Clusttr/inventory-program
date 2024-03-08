// use crate::utils::*;
use crate::state::{Inventory, InventoryAccount};
use crate::utils::InventoryError;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
// use anchor_spl::token_interface::TokenAccount;
// use std::ops::Mul;

#[account]
pub struct AssetInfo {
    pub asset_key: Pubkey,
    pub price: u64,
    pub amount: u64,
    pub usdc_remit_account: Pubkey,
}

impl AssetInfo {
    pub fn new(asset_key: Pubkey, price: u64, usdc_remit_account: Pubkey) -> Self {
        // let usdc_price: f64 = price.mul(10.pow(decimals::USDC));
        Self {
            asset_key,
            price,
            amount: 0,
            usdc_remit_account,
        }
    }
}

impl AssetInfo {
    pub const SPACE: usize = std::mem::size_of::<AssetInfo>();
    pub const SEED_PREFIX: &'static str = "asset_info";
}

pub trait AssetInfoAccount<'info> {
    fn add(
        &mut self,
        deposit: (
            &Account<'info, Mint>,         //mint
            &Account<'info, TokenAccount>, //from
            &Account<'info, TokenAccount>, //to
            u64,                           //amount
        ),
        authority: &Signer<'info>,
        inventory: &mut Account<'info, Inventory>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;
}

impl<'info> AssetInfoAccount<'info> for Account<'info, AssetInfo> {
    fn add(
        &mut self,
        deposit: (
            &Account<'info, Mint>,         //mint
            &Account<'info, TokenAccount>, //from
            &Account<'info, TokenAccount>, //to
            u64,                           //amount
        ),
        authority: &Signer<'info>,
        inventory: &mut Account<'info, Inventory>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let (mint, from, to, amount) = deposit;
        //check if asset is in inventory
        inventory.check_asset(&mint.key())?;

        //confirm if user has that amount of asset
        if from.amount < amount {
            return Err(InventoryError::InsufficientAsset.into());
        }

        //transfer asset
        transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;

        //update asset_info
        self.amount += amount;
        Ok(())
    }
}
