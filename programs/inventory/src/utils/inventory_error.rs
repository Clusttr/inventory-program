use anchor_lang::prelude::*;

#[error_code]
pub enum InventoryError {
    #[msg("An invalid asset mint address provided")]
    InvalidAssetId,
}
