use ink::prelude::vec::Vec;
use openbrush::contracts::psp34::Id;

use crate::impls::staking::types::StakingError;

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens
    #[ink(message)]
    fn stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// un-stake chickens tokens
    #[ink(message)]
    fn request_un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// un-stake chickens tokens
    #[ink(message)]
    fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;
}
