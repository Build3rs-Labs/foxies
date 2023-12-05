#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod swap {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

    use ink::storage::Mapping;

    use ink::prelude::vec;

    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment
        }
    };

    use ink::contract_ref;

    use psp22::PSP22;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum SwapError {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        /// Thrown when there isn't sufficient liquidity
        InsufficientLiquidity
    }

    #[ink(storage)]
    pub struct Swap {

    }

    impl Swap {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self {

            }
        }
        
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            Self::env().account_id()
        }

    }
}