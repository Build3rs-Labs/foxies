use openbrush::traits::Balance;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // 15,000 is total supply
    pub total_supply: u64,
    // 90% i.e. (15000 * 90 % which is 13,500)
    pub chickens_supply: u64,
    // 10% i.e. (1,500 * 10 % which is 1,500)
    pub foxes_supply: u64,
    // token_id starts from 0
    pub last_token_id: u64,
    // Balance to mint chickens and foxes token
    pub price_per_mint: Balance,
    pub _reserved: Option<()>,
}