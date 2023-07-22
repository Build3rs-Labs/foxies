#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use foxies::{impls::mint_token::*, traits::mint_token::*};
    use ink::env::hash;
    use ink::prelude::vec::Vec;
    use openbrush::traits::Storage;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        foxies: types::Data,
        random_number: u64, // random number from `generate_random_number` function
        salt: u64,
        max_value: u64, // maximum value
    }

    // impl PSP34 for Contract {}
    impl PayableMint for Contract {}

    impl Contract {
        // Add max value while contract uploading
        #[ink(constructor)]
        pub fn new(max_value: u64) -> Self {
            let mut instance = Self::default();
            instance.max_value = max_value;
            instance
        }

        // This function generates a hash value that is based on the block timestamp and the incremented salt value.
        // The max_value is used to determine the maximum value in the range.
        // The modulo operator % (max_value + 1) is then used to return a number between 0 and the maximum value in the range.
        #[ink(message)]
        pub fn generate_random_number(&mut self) -> u64 {
            let seed = self.env().block_timestamp();
            let mut input: Vec<u8> = Vec::new();
            input.extend_from_slice(&seed.to_be_bytes());
            input.extend_from_slice(&self.salt.to_be_bytes());
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
            self.salt += 1;
            let number_bytes = [output[0], output[1]];
            let z = u16::from_be_bytes(number_bytes);
            let number = z as u64 % (self.max_value + 1);
            self.random_number = number;
            number
        }

        // Returns random generated number
        #[ink(message)]
        pub fn get_random_number(&self) -> u64 {
            self.random_number
        }

        #[ink(message)]
        pub fn get_max_value(&self) -> u64 {
            self.max_value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_pseudo_random() {
            let max_value: u64 = 15000;
            let mut contract = Contract::new(max_value);
            let result = contract.generate_random_number();
            assert!(result <= max_value);
        }
    }
}
