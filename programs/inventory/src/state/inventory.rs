use crate::utils::InventoryError;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::Mint;
use std::ops::{Div, Mul};

#[account]
pub struct Inventory {
    pub assets: Vec<Pubkey>,
}

impl Inventory {
    pub const SPACE: usize = 8 + 4 + 1;
    pub const SEED_PREFIX: &'static str = "inventory";

    pub fn new() -> Self {
        Self { assets: vec![] }
    }
}

pub trait InventoryAccount<'info> {
    fn check_asset(&mut self, asset_id: &Pubkey) -> Result<()>;

    fn insert_asset(
        &mut self,
        asset_mint: &Account<'info, Mint>,
        payer: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn remove_asset(&mut self, asset_mint: &Account<'info, Mint>) -> Result<()>;

    fn realloc(
        &mut self,
        space: usize,
        payer: &Signer<'info>,
        system_program: Program<'info, System>,
    ) -> Result<()>;
}

impl<'info> InventoryAccount<'info> for Account<'info, Inventory> {
    fn check_asset(&mut self, asset_id: &Pubkey) -> Result<()> {
        if self.assets.contains(asset_id) {
            Ok(())
        } else {
            Err(InventoryError::InvalidAssetId.into())
        }
    }

    fn insert_asset(
        &mut self,
        asset_mint: &Account<'info, Mint>,
        payer: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        let asset_key = asset_mint.key();

        match self.check_asset(&asset_key) {
            Ok(_) => {}
            Err(_) => {
                let space = std::mem::size_of::<Pubkey>();
                self.realloc(space, payer, system_program.clone())?;
                self.assets.push(asset_key)
            }
        }
        Ok(())
    }

    fn remove_asset(&mut self, asset_mint: &Account<'info, Mint>) -> Result<()> {
        let asset_key = asset_mint.key();
        match self.check_asset(&asset_key) {
            Ok(_) => {
                self.assets.retain(|key| key != &asset_mint.key());
                Ok(())
            }
            Err(_) => Err(InventoryError::InvalidAssetId.into()),
        }
    }

    fn realloc(
        &mut self,
        space: usize,
        payer: &Signer<'info>,
        system_program: Program<'info, System>,
    ) -> Result<()> {
        let account_info = self.to_account_info();
        let new_account_size = account_info.data_len() + space;
        let lamport_required = (Rent::get())?.minimum_balance(new_account_size);
        let additional_rent_to_pay = lamport_required - account_info.lamports();
        transfer_lamports(
            payer,
            account_info.clone(),
            additional_rent_to_pay,
            system_program,
        )?;
        account_info.realloc(new_account_size, false)?;
        Ok(())
    }
}

fn transfer_lamports<'info>(
    from: &Signer<'info>,
    to: AccountInfo<'info>,
    amount: u64,
    system_program: Program<'info, System>,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: from.to_account_info(),
                to,
            },
        ),
        amount,
    )
}

/// Converts a `u64` value - in this case the balance of a token account - into
/// an `f32` by using the `decimals` value of its associated mint to get the
/// nominal quantity of a mint stored in that token account
///s
/// For example, a token account with a balance of 10,500 for a mint with 3
/// decimals would have a nominal balance of 10.5
fn _convert_to_float(value: u64, decimals: u8) -> f32 {
    (value as f32).div(f32::powf(10.0, decimals as f32))
}

/// Converts a nominal value - in this case the calculated value `r` - into a
/// `u64` by using the `decimals` value of its associated mint to get the real
/// quantity of the mint that the user will receive
///
/// For example, if `r` is calculated to be 10.5, the real amount of the asset
/// to be received by the user is 10,500
fn _convert_from_float(value: f32, decimals: u8) -> u64 {
    value.mul(f32::powf(10.0, decimals as f32)) as u64
}
