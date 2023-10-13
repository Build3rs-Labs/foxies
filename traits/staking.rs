use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
    },
};

use crate::impls::staking::types::StakingError;

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens

    #[ink(message)]
    fn stake(&mut self, token_id: Id) -> Result<(), StakingError>;

    #[ink(message)]
    fn un_stake(&mut self, token_id: Id) -> Result<(), StakingError>;

    /// claim rewards
    #[ink(message)]
    fn claim_token_rewards(&mut self, account: AccountId, item: Id) -> Result<(), StakingError>;

    #[ink(message)]
    fn set_token_earn_per_day(&mut self, amount_of_eggs_token_earn_per_day: Balance) -> Result<(), StakingError>;

    /// This function returns the total NFT Staked by an account
    #[ink(message)]
    fn get_total_staked_by_account(&self, account: AccountId) -> u64;

    #[ink(message)]
    fn get_staked_item_days(&self, account: AccountId, item: Id) -> u64;
}
