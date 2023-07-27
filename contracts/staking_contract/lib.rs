#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod staking_contract {
    use foxies::{impls::staking::*, traits::staking::*};
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingContract {
        #[storage_field]
        foxies: types::Data,
    }

    impl Staking for StakingContract {}

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(nft_contract_address: AccountId, eggs_token_address: AccountId) -> Self {
            let mut instance = Self::default();
            instance.foxies.nft_contract_address = nft_contract_address;
            instance.foxies.eggs_token_address = eggs_token_address;
            instance
        }
    }
}
