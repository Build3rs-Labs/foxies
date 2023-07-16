use crate::impls::staking::types::Data;
pub use crate::traits::staking::Staking;
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::{Id, PSP34Error},
    traits::{AccountId, Storage},
};

impl<T> Staking for T
where
    T: Storage<Data>,
{
    default fn stake_chickens(&mut self, token_id: Vec<Id>) -> Result<(), PSP34Error> {
        Ok(())
    }
    default fn un_stake_chickens(&mut self, token_id: Vec<Id>) -> Result<(), PSP34Error> {
        Ok(())
    }

    default fn get_total_staked_chickens_by_account(&self, account: AccountId) -> u64 {
        self.data::<Data>()
            .total_staked_token_by_account
            .get(account)
            .unwrap_or_default()
    }
}
