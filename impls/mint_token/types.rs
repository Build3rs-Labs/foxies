use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{
        AccountId,
        Balance,
        String,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub owner: AccountId,
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

impl Default for Data {
    fn default() -> Self {
        Data {
            owner: ZERO_ADDRESS.into(),
            max_supply: Default::default(),
            price_per_mint: Default::default(),
            salt: Default::default(),
            random_number: Default::default(),
            _reserved: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MintTokenError {
    RepetativeRandomNumber,
    BadMintValue,
    InvalidAccount,
    OwnerCantMint,
}

impl From<MintTokenError> for PSP34Error {
    fn from(value: MintTokenError) -> Self {
        match value {
            MintTokenError::RepetativeRandomNumber => PSP34Error::Custom(String::from("RepetativeRandomNumber")),
            MintTokenError::BadMintValue => PSP34Error::Custom(String::from("BadMintValue")),
            MintTokenError::InvalidAccount => PSP34Error::Custom(String::from("InvalidAccount")),
            MintTokenError::OwnerCantMint => PSP34Error::Custom(String::from("OwnerCantMint")),
        }
    }
}
