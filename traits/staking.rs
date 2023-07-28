use ink::prelude::vec::Vec;
use openbrush::{contracts::psp34::Id, traits::AccountId};

use crate::impls::staking::types::StakingError;

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens
    #[ink(message)]
    fn stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// request_un-stake chickens tokens
    #[ink(message)]
    fn request_un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// un-stake chickens tokens
    #[ink(message)]
    fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError>;

    /// claim rewards
    #[ink(message)]
    fn claim_rewards(&mut self) -> Result<(), StakingError>;

    /// Set Account so it can claim the reward. Must run by backend every month before add_reward
    #[ink(message)]
    fn set_claimed_status(&mut self, staker: AccountId) -> Result<(), StakingError>;
}
