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

   

    /// Get random number in vector
    #[ink(message)]
    fn get_random_numbers_vector(&self) -> Vec<u64>;
}