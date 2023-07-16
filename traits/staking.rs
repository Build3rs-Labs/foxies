use openbrush::{contracts::psp34::{Id, PSP34Error}, traits::AccountId};
use ink::prelude::vec::Vec;

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens
    #[ink(message, payable)]
    fn stake_chickens(&mut self, token_id: Vec<Id>) -> Result<(), PSP34Error>;

    /// un-stake chickens tokens
    #[ink(message, payable)]
    fn un_stake_chickens(&mut self,  token_id: Vec<Id>) -> Result<(), PSP34Error>;

    /// This function returns the total PMP NFT Staked by an account
    #[ink(message)]
    fn get_total_staked_chickens_by_account(&self, account: AccountId) -> u64;
}
