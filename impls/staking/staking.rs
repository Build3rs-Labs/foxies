pub use crate::traits::staking::Staking;
use crate::{
    ensure,
    helpers::constants::{
        DAY_IN_UNIX_TIME,
        FOXES_RANGE,
    },
    impls::staking::types::Data,
};
use ink::{
    env::{
        hash,
        CallFlags,
    },
    prelude::{
        vec,
        vec::Vec,
    },
};
use openbrush::{
    contracts::{
        psp22::*,
        psp34::{
            Id,
            *,
        },
    },
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Storage,
    },
};

use super::types::StakingError;

impl<T> Staking for T
where
    T: Storage<Data>,
{
    default fn stake(&mut self, token_id: Id) -> Result<(), StakingError> {
        let caller = Self::env().caller();

        if let Some(total_staked) = self.data::<Data>().total_staked.checked_add(1) {
            self.data::<Data>().total_staked = total_staked;

            // step 0 - check if the token is chicken or not
            if token_id < Id::U64(FOXES_RANGE) {
                return Err(StakingError::CantStakeFoxesToken)
            }

            let contract_address = self.data::<Data>().nft_contract_address;
            self.check_token_owner(contract_address, token_id.clone())?;

            // step 2 - check if the contract has been approved
            let allowance = PSP34Ref::allowance(
                &self.data::<Data>().nft_contract_address,
                caller,
                Self::env().account_id(),
                Some(token_id.clone()),
            );

            if !allowance {
                return Err(StakingError::NotApproved)
            }

            // insert stake token_id inside `staking_list` mapping (key -> caller, value -> token_id)
            self.data::<Data>().staking_list.insert(caller, &token_id);

            // step3 - transfer token_id owner from caller to staking contract
            let builder = PSP34Ref::transfer_builder(
                &self.data::<Data>().nft_contract_address,
                Self::env().account_id(),
                token_id.clone(),
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true));

            let result = match builder.try_invoke() {
                Ok(Ok(Ok(_))) => Ok(()),
                Ok(Ok(Err(e))) => Err(e.into()),
                Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
                Err(ink::env::Error::NotCallable) => Ok(()),
                _ => Err(StakingError::CannotTransfer),
            };

            if result.is_ok() {
                // update staking start time of caller's token id inside `staking_start_time` mapping
                let start_time = Self::env().block_timestamp();

                self.data::<Data>()
                    .staking_start_time
                    .insert((caller, token_id.clone()), &start_time);

                // update staked_accounts status to 0 which is staked status
                if !self.data::<Data>().staked_accounts.contains_value(0, &caller) {
                    self.data::<Data>().staked_accounts.insert(0, &caller);
                }

                self.emit_stake_token_event(caller, token_id);
            } else {
                return Err(StakingError::CannotFindTokenOwner)
            }
        } else {
            return Err(StakingError::FailedToIncreaseTotalStaked)
        }
        Ok(())
    }

    default fn un_stake(&mut self, token_id: Id) -> Result<(), StakingError> {
        let caller = Self::env().caller();

        // step 1 - check token owner is contract staking
        // let contract_address = self.data::<Data>().nft_contract_address;
        // self.check_token_owner(contract_address, token_id.clone())?;
        if let Some(token_owner) = PSP34Ref::owner_of(&self.data::<Data>().nft_contract_address, token_id.clone()) {
            if Self::env().account_id() != token_owner {
                return Err(StakingError::InvalidCaller)
            }

            // step 2 - check staker (i.e caller) has stake token
            if !self
                .data::<Data>()
                .staking_list
                .contains_value(caller, &token_id.clone())
            {
                return Err(StakingError::InvalidInput)
            }

            // step 3 - remove token from `staking_list`
            self.data::<Data>().staking_list.remove_value(caller, &token_id.clone());

            // step 4 - transfer token to caller
            if PSP34Ref::transfer(
                &self.data::<Data>().nft_contract_address,
                caller,
                token_id.clone(),
                Vec::<u8>::new(),
            )
            .is_err()
            {
                return Err(StakingError::CannotTransfer)
            }

            // step 4 - calculate total staking time of token

            let staking_item_time = self
                .data::<Data>()
                .staking_start_time
                .get(&(caller, token_id.clone()))
                .unwrap_or_default();
            let unstaking_item_time = Self::env().block_timestamp();

            let time_difference = unstaking_item_time.checked_sub(staking_item_time).unwrap_or_default();

            // Convert the time difference to the number of days
            // convert the time difference to the number of days
            // https://www.unixtimestamp.com
            // 7 days = 604800000 in Unix time
            // let day = 86400000;

            let days_staked = time_difference.checked_div(DAY_IN_UNIX_TIME).unwrap_or_default();

            // Add nft staking days
            self.data::<Data>()
                .nft_staking_days
                .insert(&(caller, token_id.clone()), &days_staked);

            self.emit_request_unstake_token_event(caller, token_id);

            // if not token found in `staking_list`, then remove staked_accounts value
            if self.data::<Data>().staking_list.count(caller) == 0 {
                self.data::<Data>().staked_accounts.remove_value(0, &caller);
            }

            // if staked_accounts contains no value, then update
            if !self.data::<Data>().staked_accounts.contains_value(1, &caller) {
                self.data::<Data>().staked_accounts.insert(1, &caller);
            }

            // substract `leng` item length from total_staked
            if let Some(total_staked) = self.data::<Data>().total_staked.checked_sub(1) {
                self.data::<Data>().total_staked = total_staked;
            } else {
                return Err(StakingError::FailedToDescreaseTotalStaked)
            }
        }

        Ok(())
    }

    default fn get_staked_item_days(&self, account: AccountId, item: Id) -> u64 {
        self.data::<Data>()
            .nft_staking_days
            .get((account, item))
            .unwrap_or_default()
    }

    default fn claim_token_rewards(&mut self, account: AccountId, item: Id) -> Result<(), StakingError> {
        if let Some(days) = self.data::<Data>().nft_staking_days.get((account, item)) {
            if days > 0 {
                let amount_of_eggs_token_earn_per_day = self.data::<Data>().amount_of_eggs_token_earn_per_day;

                // number of $Eggs token earn by account as per staking days
                let earn_by_staking = days as u128 * amount_of_eggs_token_earn_per_day;

                // Stealing mechanism : We define the stealing mechanism function. When a chicken tries to claim their $EGGS,
                // there should be a chance that a fox can steal some or all of it.
                // get random foxes NFT token (foxes NFT token between 0 - 1500)
                let random_foxes_nft = self.random_number(FOXES_RANGE).unwrap_or_default();
                // get random foxes NFT token owner who steal some or all of caller `$Eggs` token
                if let Some(random_foxes_nft_owner) =
                    PSP34Ref::owner_of(&self.data::<Data>().nft_contract_address, Id::U64(random_foxes_nft))
                {
                    // get random number between 0 and `earn_by_staking` to steal some or all of it
                    let steal_some_or_all_eggs = self.random_number(earn_by_staking as u64).unwrap_or_default();

                    // check if all eggs are transferring or just some eggs
                    if steal_some_or_all_eggs == earn_by_staking as u64 {
                        // transfer all EGGS token to random_foxes_nft_owner wallet
                        PSP22Ref::transfer(
                            &self.data::<Data>().eggs_token_address,
                            random_foxes_nft_owner,
                            earn_by_staking,
                            vec![],
                        )
                        .unwrap_or_default();
                    } else {
                        // transfer some eggs to random_foxes_nft_owner wallet
                        PSP22Ref::transfer(
                            &self.data::<Data>().eggs_token_address,
                            random_foxes_nft_owner,
                            steal_some_or_all_eggs as u128,
                            vec![],
                        )
                        .unwrap_or_default();

                        // transfer remaining eggs to token owner wallet
                        PSP22Ref::transfer(
                            &self.data::<Data>().eggs_token_address,
                            account,
                            earn_by_staking - steal_some_or_all_eggs as u128,
                            vec![],
                        )
                        .unwrap_or_default();
                    }
                    self.claim_request_event(account, earn_by_staking as u64 - steal_some_or_all_eggs);
                    Ok(())
                } else {
                    return Err(StakingError::RandomFoxesNFTNotFound)
                }
            } else {
                return Err(StakingError::CannotTransfer)
            }
        } else {
            return Err(StakingError::InvalidTime)
        }
    }

    #[modifiers(contract_owner)]
    default fn set_token_earn_per_day(
        &mut self,
        amount_of_eggs_token_earn_per_day: Balance,
    ) -> Result<(), StakingError> {
        self.data::<Data>().amount_of_eggs_token_earn_per_day = amount_of_eggs_token_earn_per_day;
        Ok(())
    }

    // Get User NFT staked in the contract
    default fn get_total_staked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Data>().staking_list.count(account) as u64
    }
}

pub trait Internal {
    // fn get_request_unstake_time(&self, account: AccountId, token_id: Id) -> u64;
    // When a chicken tries to claim their $EGGS, there should be a chance that a fox can steal some or all of it
    fn setaling_eggs(&self) -> Result<(), StakingError>;
    // max_value: FOXES_RANGE
    fn random_number(&mut self, max_value: u64) -> Result<u64, StakingError>;
    /// Checks if contract caller is an token owner
    fn check_token_owner(&self, nft_contract_address: AccountId, token_id: Id) -> Result<(), StakingError>;
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn check_token_owner(&self, nft_contract_address: AccountId, token_id: Id) -> Result<(), StakingError> {
        let caller = Self::env().caller();
        match PSP34Ref::owner_of(&nft_contract_address, token_id) {
            Some(token_owner) => {
                ensure!(caller == token_owner, StakingError::NotTokenOwner);
                Ok(())
            }
            None => Err(StakingError::CannotFindTokenOwner),
        }
    }

    // When a chicken tries to claim their $EGGS, there should be a chance that a fox can steal some or all of it
    default fn setaling_eggs(&self) -> Result<(), StakingError> {
        Ok(())
    }

    default fn random_number(&mut self, max_value: u64) -> Result<u64, StakingError> {
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
        let random_number = z as u64 % (max_value + 1);

        Ok(random_number)
    }
}

// Events of TokenStaking
pub trait TokenStakingEvents {
    fn emit_stake_token_event(&self, owner: AccountId, item_id: Id);
    fn emit_request_unstake_token_event(&self, owner: AccountId, item_id: Id);
    fn emit_cancel_request_unstake_token_event(&self, owner: AccountId, item_id: Id);
    fn emit_unstake_token_event(&self, owner: AccountId, item_id: Id);
    fn claim_request_event(&self, owner: AccountId, reward: u64);
}

impl<T> TokenStakingEvents for T
where
    T: Storage<Data>,
{
    default fn emit_stake_token_event(&self, _owner: AccountId, _item_id: Id) {}
    default fn emit_request_unstake_token_event(&self, _owner: AccountId, _item_id: Id) {}
    default fn emit_cancel_request_unstake_token_event(&self, _owner: AccountId, _item_id: Id) {}
    default fn emit_unstake_token_event(&self, _owner: AccountId, _item_id: Id) {}
    default fn claim_request_event(&self, _owner: AccountId, _reward: u64) {}
}

#[modifier_definition]
pub fn contract_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<StakingError>,
{
    // AccountId mustn't be zero
    ensure!(!T::env().caller().is_zero(), StakingError::InvalidAccount);
    // Only contract owner can call
    ensure!(
        T::env().caller() != instance.data().admin_address,
        StakingError::NotContractOwner
    );
    body(instance)
}
