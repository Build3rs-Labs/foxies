#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use foxies::{impls::mint_token::*, traits::mint_token::*};
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
        },
        traits::Storage,
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
}
