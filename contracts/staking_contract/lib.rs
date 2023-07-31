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
        pub fn new(
            admin_address: AccountId,
            nft_contract_address: AccountId,
            eggs_token_address: AccountId,
            limit_unstaking_time: u64,
            amount_of_eggs_token_earn_per_day: Balance,
        ) -> Self {
            let mut instance = Self::default();
            instance.foxies.admin_address = admin_address;
            instance.foxies.nft_contract_address = nft_contract_address;
            instance.foxies.eggs_token_address = eggs_token_address;
            instance.foxies.limit_unstaking_time = limit_unstaking_time;
            instance.foxies.amount_of_eggs_token_earn_per_day = amount_of_eggs_token_earn_per_day;
            instance
        }
    }
}
