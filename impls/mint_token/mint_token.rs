use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{AccountId, Balance, Storage},
};

use crate::impls::mint_token::types::Data;
pub use crate::traits::mint_token::PayableMint;
use ink::prelude::string::String as PreludeString;

impl<T> PayableMint for T
where
    T: Storage<Data>,
{
    default fn mint_chickens(&mut self, to: AccountId) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn mint_foxes(&mut self, to: AccountId) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn get_base_uri(&self, token_id: u64) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn total_supply(&self) -> u64 {
        self.data::<Data>().total_supply
    }
    default fn price(&self) -> Balance {
        self.data::<Data>().price_per_mint
    }
}
