use ink::prelude::vec::Vec;
use openbrush::traits::{Balance, String};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // 15,000 is max supply
    pub max_supply: u64,
    // amount to mint chickens and foxes token
    pub price_per_mint: Balance,
    // `salt` is variable based on which random number generated
    // `salt` incremented each time random number is generated
    pub salt: u64,
    // random number
    pub random_number: Vec<u64>,
    pub _reserved: Option<()>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MintTokenError {
    RepetativeRandomNumber,
    BadMintValue,
}

impl MintTokenError {
    pub fn as_str(&self) -> String {
        match self {
            MintTokenError::RepetativeRandomNumber => String::from("RepetativeRandomNumber"),
            MintTokenError::BadMintValue => String::from("BadMintValue"),
        }
    }
}