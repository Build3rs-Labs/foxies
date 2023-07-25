use ink::prelude::vec::Vec;
use openbrush::{contracts::psp34::Id, traits::AccountId};

use crate::impls::staking::types::StakingError;


#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens
    #[ink(message, payable)]
    fn stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// un-stake chickens tokens
    #[ink(message, payable)]
    fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// This function returns the total PMP NFT Staked by an account
    #[ink(message)]
    fn get_total_staked_chickens_by_account(&self, account: AccountId) -> u64;

    /// This function returns the total PMP NFT Staked by an account
    #[ink(message)]
    fn get_staking_list_token(&self, account: AccountId) -> Vec<Id>;
}
