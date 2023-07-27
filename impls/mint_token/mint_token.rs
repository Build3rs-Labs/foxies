use crate::impls::mint_token::types::{Data, MintTokenError};
pub use crate::traits::mint_token::PayableMint;
use ink::env::hash;
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::{extensions::enumerable::*, Id, PSP34Error},
    traits::{AccountId, Storage},
};
impl<T> PayableMint for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    default fn mint_token(&mut self, to: AccountId) -> Result<(), PSP34Error> {
        self.check_value(Self::env().transferred_value(), 1)?;
        // Caller of the contract
        let caller = Self::env().caller();
        // Random number generated using `generate_random_number()` function (i.e. token_id)
        let random_number = self.generate_random_number().unwrap_or_default();
        // mint token
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(caller, Id::U64(random_number))?;
        Ok(())
    }

    default fn get_random_numbers_vector(&self) -> Vec<u64> {
        self.data::<Data>().random_number.clone()
    }
}

pub trait Internal {
    // This function generates a hash value that is based on the block timestamp and the incremented salt value.
    // The max_value is used to determine the maximum value in the range.
    // The modulo operator % (max_value + 1) is then used to return a number between 0 and the maximum value in the range.
    fn generate_random_number(&mut self) -> Result<u64, MintTokenError>;

    // Check if the transferred mint values is as expected
    fn check_value(&self, transferred_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    // Check if token is minted
    fn token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn generate_random_number(&mut self) -> Result<u64, MintTokenError> {
        let seed = Self::env().block_timestamp();
        // Define mutable empty vector
        let mut input: Vec<u8> = Vec::new();
        // `extend_from_slice()` Clones and appends all elements in a slice to the Vec
        // `to_be_bytes()` Return the memory representation of this integer as a byte array in big-endian (network) byte order.
        input.extend_from_slice(&seed.to_be_bytes());
        input.extend_from_slice(&self.data::<Data>().salt.to_be_bytes());
        // `hash` Provides type definitions and traits for the built-in cryptographic hashes.
        // `keccak256` The KECCAK crypto hash with 256-bit output.
        // `HashOutput` The output type of built-in cryptographic hash functions.
        let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
        // `hash_bytes` Conducts the crypto hash of the given input and stores the result in output.
        // and takes two arguments: the input and the output
        ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
        // increase `self.data<Data>().salt` by 1
        self.data::<Data>().salt += 1;
        // if we use just `output[0]` then we can't use value more than `u8::MAX`
        // to use more bits we can make `number_bytes` like this
        let number_bytes = [output[0], output[1]];
        let z = u16::from_be_bytes(number_bytes);
        let random_number = z as u64 % (self.data::<Data>().max_supply + 1);
        // Check if random number is repetative or not
        let contains_random_number = self.data::<Data>().random_number.contains(&random_number);

        // `match` check Control flow based on pattern matching.
        match contains_random_number {
            // if repetative then return error
            true => return Err(MintTokenError::RepetativeRandomNumber),
            // otherwise add random number to random_number vector
            false => self.data::<Data>().random_number.push(random_number),
        };

        // otherwise return success
        Ok(random_number)
    }

    default fn check_value(
        &self,
        transferred_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transferred_value == value {
                // TODO: need to transfer the value to owner account
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(MintTokenError::BadMintValue.as_str()));
    }

    default fn token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        Ok(())
    }
}
