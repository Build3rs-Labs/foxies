use ink::prelude::{string::String as PreludeString, vec::Vec};
use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type PayableMintRef = dyn PayableMint;

#[openbrush::trait_definition]
pub trait PayableMint {
    /// Mint chickens or foxes tokens
    #[ink(message, payable)]
    fn mint_token(&mut self, to: AccountId) -> Result<(), PSP34Error>;

    /// Set new value for the baseUri
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;

    /// Get URI from token ID
    #[ink(message)]
    fn get_base_uri(&self, token_id: u64) -> Result<(), PSP34Error>;

    /// Get max supply of tokens
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// Get token price
    #[ink(message)]
    fn price(&self) -> Balance;

    /// Get random number in vector
    #[ink(message)]
    fn get_random_numbers_vector(&self) -> Vec<u64>;
}