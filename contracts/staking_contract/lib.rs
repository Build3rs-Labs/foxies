#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod staking_contract {
    use foxies::impls::staking::staking::TokenStakingEvents;
    use foxies::{impls::staking::*, traits::staking::*};
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use openbrush::{contracts::psp34::Id, traits::Storage};
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

    #[ink(event)]
    pub struct StakeEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: Id,
    }

    #[ink(event)]
    pub struct RequestUnStakeEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: Id,
    }

    #[ink(event)]
    pub struct CancelRequestUnStakeEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: Id,
    }

    #[ink(event)]
    pub struct UnStakeEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: Id,
    }

    #[ink(event)]
    pub struct ClaimRewardEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        reward: u64,
    }

    impl TokenStakingEvents for StakingContract {
        fn emit_stake_token_event(&self, owner: AccountId, item_id: Id) {
            self.env().emit_event(StakeEvent { owner, item_id });
        }
        fn emit_request_unstake_token_event(&self, owner: AccountId, item_id: Id) {
            self.env()
                .emit_event(RequestUnStakeEvent { owner, item_id });
        }
        fn emit_cancel_request_unstake_token_event(&self, owner: AccountId, item_id: Id) {
            self.env()
                .emit_event(CancelRequestUnStakeEvent { owner, item_id });
        }
        fn emit_unstake_token_event(&self, owner: AccountId, item_id: Id) {
            self.env().emit_event(UnStakeEvent { owner, item_id });
        }
        fn claim_reqard_event(&self, owner: AccountId, reward: u64) {
            self.env().emit_event(ClaimRewardEvent { owner, reward });
        }
    }
}
