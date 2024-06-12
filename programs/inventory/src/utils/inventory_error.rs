use anchor_lang::prelude::*;

#[error_code]
pub enum InventoryError {
    #[msg("An invalid asset functions address provided")]
    InvalidAssetId,

    #[msg("Not enough asset in inventory")]
    InsufficientInventoryAsset,

    #[msg("Not enough usdc to execute traction")]
    InsufficientUSDC,

    #[msg("Not enough asset to deposit")]
    InsufficientAsset,

    #[msg("Inventory vault not needs to be emptied")]
    InventoryVaultNotEmpty,
}
