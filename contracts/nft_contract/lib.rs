#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use foxies::impls::mint_token::mint_token::TokenMintingEvents;
    use foxies::{impls::mint_token::*, traits::mint_token::*};
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
        },
        traits::{Storage, String},
    };
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        foxies: types::Data,
    }

    #[ink(event)]
    pub struct FoxesEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: u64,
        #[ink(topic)]
        token_name: String,
    }

    #[ink(event)]
    pub struct ChickenEvent {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        item_id: u64,
        #[ink(topic)]
        token_name: String,
    }

    impl Ownable for Contract {}
    impl PSP34 for Contract {}
    impl PSP34Metadata for Contract {}
    impl PSP34Enumerable for Contract {}
    impl PayableMint for Contract {}

    impl Contract {
        // Add max value while contract uploading
        #[ink(constructor)]
        pub fn new(max_value: u64, price_per_mint: Balance) -> Self {
            let mut instance = Self::default();
            instance.foxies.max_supply = max_value;
            instance.foxies.price_per_mint = price_per_mint;
            instance
        }
    }

    impl TokenMintingEvents for Contract {
        fn emit_mint_foxes_token_event(&self, owner: AccountId, item_id: u64, token_name: String) {
            self.env().emit_event(FoxesEvent {
                owner,
                item_id,
                token_name,
            });
        }
        fn emit_mint_chicken_token_event(
            &self,
            owner: AccountId,
            item_id: u64,
            token_name: String,
        ) {
            self.env().emit_event(ChickenEvent {
                owner,
                item_id,
                token_name,
            });
        }
    }
}
