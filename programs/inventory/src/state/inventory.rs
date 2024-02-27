use crate::utils::InventoryError;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::Mint;

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
