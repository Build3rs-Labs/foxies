use openbrush::{
    contracts::psp34::{PSP34Error, Id},
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// stake chickens tokens
    #[ink(message, payable)]
    fn stake_chickens(&mut self, token_id: Id) -> Result<(), PSP34Error>;

    /// un-stake chickens tokens
    #[ink(message, payable)]
    fn un_stake_chickens(&mut self, token_id: Id) -> Result<(), PSP34Error>;
}
