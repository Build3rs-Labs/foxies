#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use foxies::{impls::mint_token::*, traits::mint_token::*};
    use openbrush::{contracts::psp34::*, traits::Storage};
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        foxies: types::Data,
    }

    impl PSP34 for Contract {}
    impl PayableMint for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
