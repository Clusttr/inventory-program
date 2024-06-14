use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[account]
pub struct AssetInfo {
    pub asset_key: Pubkey,
    pub price: u64,
    pub usdc_remit_account: Pubkey,
}

impl AssetInfo {
    pub fn new(asset_key: Pubkey, price: u64, usdc_remit_account: Pubkey) -> Self {
        // let usdc_price: f64 = price.mul(10.pow(decimals::USDC));
        Self {
            asset_key,
            price,
            // amount: 0,
            usdc_remit_account,
        }
    }
}

impl AssetInfo {
    pub const SPACE: usize = std::mem::size_of::<AssetInfo>();
    pub const SEED_PREFIX: &'static str = "asset_info";
}

pub trait AssetInfoAccount<'info> {
    fn buy(
        &mut self,
        deposit: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
        ),
        receive: (
            &Account<'info, Mint>,
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            u8,
        ),
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;
}

impl<'info> AssetInfoAccount<'info> for Account<'info, AssetInfo> {
    fn buy(
        &mut self,
        deposit: (
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
        ),
        receive: (
            &Account<'info, Mint>,
            &Account<'info, TokenAccount>,
            &Account<'info, TokenAccount>,
            u8,
        ),
        amount: u64, //amount of assets user wishes to buy
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let (user_wallet, merchant_wallet) = deposit;
        let (asset_mint,
            asset_vault,
            buyer_asset_address,
            vault_bump) = receive;

        // check if vault has enough asset
        if asset_vault.amount < amount {
            return Err(InventoryError::InsufficientInventoryAsset.into());
        }

        //calculate usd required
        let total_cost = self.price * amount;

        //check if user has enough usd
        if user_wallet.amount < total_cost {
            return Err(InventoryError::InsufficientUSDC.into());
        }

        // transfer usdc
        transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_wallet.to_account_info(),
                    to: merchant_wallet.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            total_cost,
        )?;

        // transfer asset,
        let mint_key = asset_mint.key();
        let seed: &[&[&[u8]]] = &[&[
            main_const::VAULT,
            mint_key.as_ref(),
            authority.key.as_ref(),
            &[vault_bump],
        ]];
        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: asset_vault.to_account_info(),
                    to: buyer_asset_address.to_account_info(),
                    authority: asset_vault.to_account_info(),
                },
                seed,
            ),
            amount,
        )?;
        Ok(())
    }
}
