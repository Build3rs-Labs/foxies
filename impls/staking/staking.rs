use openbrush::{contracts::psp34::{Id, PSP34Error}, traits::Storage};

use crate::impls::staking::types::Data;
pub use crate::traits::staking::Staking;

impl<T> Staking for T
where
    T: Storage<Data>,
{
    default fn stake_chickens(&mut self, token_id: Id) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn un_stake_chickens(&mut self, token_id: Id) -> Result<(), PSP34Error> {
        Ok(())
    }
}
