use anchor_lang::prelude::*;

#[error_code]
pub enum InventoryError {
    #[msg("An invalid asset functions address provided")]
    InvalidAssetId,
}
