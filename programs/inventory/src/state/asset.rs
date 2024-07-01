use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    /// Asset address on-chain
    pub id: Pubkey,
    /// Value of asset in usd
    /// note: figure is in two decimals; 100 == $1
    pub value: u64,
    /// Expected appreciation in a year in
    pub appreciation_rate: u16,
    /// Annual rental value
    pub rent: u32,
    /// Total revenue generated from asset
    pub cumulative_revenue: u64,
    /// Total cost used in maintaining asset
    pub cumulative_maintenance_cost: u64,
}

impl Asset {
    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        let asset_info_data = &mut &**account_info.try_borrow_data().unwrap();
        let asset = Asset::try_deserialize(asset_info_data).unwrap();
        return asset;
    }
}