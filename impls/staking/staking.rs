pub use crate::traits::staking::Staking;
use crate::{impls::staking::types::Data, traits::staking};
use ink::prelude::vec;
use ink::{env::CallFlags, prelude::vec::Vec, reflect::DispatchError};
use openbrush::{
    contracts::psp34::{Id, *},
    traits::{AccountId, Storage},
};

use super::types::StakingError;

impl<T> Staking for T
where
    T: Storage<Data>,
{
    default fn stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError> {
        // Caller of the contract
        let caller = Self::env().caller();
        // Length of `token_ids`
        let leng = token_ids.len();
        // start block timestamp
        let start_time = Self::env().block_timestamp();

        // `checked_add` Checked integer addition. Computes self + rhs, returning None if overflow occurred.
        if let Some(total_staked) = self.data::<Data>().total_staked.checked_add(leng as u64) {
            // Update `total_staked` in contract storage
            self.data::<Data>().total_staked = total_staked;

            for item in token_ids.iter() {
                // Step 1 - Check if the token is belong to caller
                if let Some(token_owner) =
                    PSP34Ref::owner_of(&self.data::<Data>().nft_contract_address, item.clone())
                {
                    if caller != token_owner {
                        return Err(StakingError::NotTokenOwner);
                    }

                    // Step 2 - Check if this contract has been approved
                    let allowance = PSP34Ref::allowance(
                        // nft contract address
                        &self.data::<Data>().nft_contract_address,
                        // caller of the contract
                        caller,
                        // staking contract address
                        Self::env().account_id(),
                        // nft token_id
                        Some(item.clone()),
                    );

                    if !allowance {
                        return Err(StakingError::NotApproved);
                    }

                    // Insert staking item in mapping with respect to caller
                    self.data::<Data>().staking_list.insert(caller, item);

                    // Step 3 - Transfer Token from Caller to staking contract
                    let builder = PSP34Ref::transfer_builder(
                        &self.data::<Data>().nft_contract_address,
                        Self::env().account_id(),
                        item.clone(),
                        // Initialize empty vector
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
                        if self.data::<Data>().is_claimed.get(&caller).is_none() {
                            self.data::<Data>().is_claimed.insert(&caller, &false);
                        }

                        // Update staking start time of caller's token id
                        self.data::<Data>()
                            .staking_start_time
                            .insert(&(caller, item.clone()), &start_time);

                        // TODO: emit event
                    }
                } else {
                    return Err(StakingError::CannotFindTokenOwner);
                }
            }

            // Update staked_accounts status to 0 which is staked status
            if !self
                .data::<Data>()
                .staked_accounts
                .contains_value(0, &caller)
            {
                self.data::<Data>().staked_accounts.insert(0, &caller);
            }
            Ok(())
        } else {
            return Err(StakingError::FailedToIncreaseTotalStaked);
        }
    }

    default fn request_un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError> {
        let caller = Self::env().caller();
        let leng = token_ids.len();

        for item in token_ids.iter() {
            // Step 1 - Check owner token is Contract Staking
            if let Some(token_owner) =
                PSP34Ref::owner_of(&self.data::<Data>().nft_contract_address, item.clone())
            {
                if Self::env().account_id() != token_owner {
                    return Err(StakingError::InvalidCaller);
                }

                // Step 2 - Check staker stake token
                if !self
                    .data::<Data>()
                    .staking_list
                    .contains_value(caller, &item.clone())
                {
                    return Err(StakingError::InvalidInput);
                }

                // Step 3 - Remove token from `staking_list`
                self.data::<Data>()
                    .staking_list
                    .remove_value(caller, &item.clone());

                // Step 4 - Update `request_unstaking_time` to current time
                let current_time = Self::env().block_timestamp();
                self.data::<Data>()
                    .request_unstaking_time
                    .insert(&(caller, item.clone()), &current_time);

                // Step 5 - Add token to pending unstaking list
                self.data::<Data>()
                    .pending_unstaking_list
                    .insert(caller, &item.clone());

                // TODO: emit_event
            } else {
                return Err(StakingError::CannotFindTokenOwner);
            }
        }

        // if not token found in staking_list, then remove staked_accounts value
        if self.data::<Data>().staking_list.count(caller) == 0 {
            self.data::<Data>().staked_accounts.remove_value(0, &caller);
        }

        // if staked_accounts contains no value then, update
        if !self
            .data::<Data>()
            .staked_accounts
            .contains_value(1, &caller)
        {
            self.data::<Data>().staked_accounts.insert(1, &caller);
        }

        // substract `leng` item length from total_staked
        if let Some(total_staked) = self.data::<Data>().total_staked.checked_sub(leng as u64) {
            self.data::<Data>().total_staked = total_staked;
            Ok(())
        } else {
            return Err(StakingError::FailedToDescreaseTotalStaked);
        }
    }

    default fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError> {
        // Step 1 - Check if the token is belong to caller and listed in pending_unstaking_list
        let caller = Self::env().caller();
        if self.data::<Data>().pending_unstaking_list.count(caller) == 0 {
            return Err(StakingError::InvalidInput);
        }

        for item in token_ids.iter() {
            // Step 2 - Check request unstaked and time request unstaked
            if !self
                .data::<Data>()
                .pending_unstaking_list
                .contains_value(caller, &item.clone())
            {
                return Err(StakingError::InvalidInput);
            }

            let request_unstake_time = self.get_request_unstake_time(caller, item.clone());
            if request_unstake_time == 0 {
                return Err(StakingError::InvalidTime);
            }

            let current_time = Self::env().block_timestamp();
            if let Some(checked_mul_value) =
                // 1 min = 60000 milliseconds
                self.data::<Data>().limit_unstaking_time.checked_mul(60000)
            {
                if let Some(unstake_time) = request_unstake_time.checked_add(checked_mul_value) {
                    if unstake_time > current_time {
                        return Err(StakingError::NotEnoughtTimeToRequestUnstake);
                    }

                    // Step 3 - transfer token to caller
                    if PSP34Ref::transfer(
                        &self.data::<Data>().nft_contract_address,
                        caller,
                        item.clone(),
                        Vec::<u8>::new(),
                    )
                    .is_err()
                    {
                        return Err(StakingError::CannotTransfer);
                    }

                    // Step 4 - Remove from pending_unstaking_list
                    self.data::<Data>()
                        .pending_unstaking_list
                        .remove_value(caller, &item.clone());

                    // Caclulate how many days he staked his nft
                    let staking_item_time = self
                        .data::<Data>()
                        .staking_start_time
                        .get(&(caller, item.clone()));
                    let unstaking_item_time = self
                        .data::<Data>()
                        .request_unstaking_time
                        .get(&(caller, item.clone()));

                    let time_difference = unstaking_item_time
                        .unwrap_or_default()
                        .checked_sub(staking_item_time.unwrap_or_default());

                    // Convert the time difference to the number of days
                    let days_staked = time_difference
                        .unwrap_or_default()
                        .checked_div(86400)
                        .unwrap_or_default();

                    // Add nft staking days
                    let mut days = self
                        .data::<Data>()
                        .nft_staking_days
                        .get(&(caller, item.clone()))
                        .unwrap_or_default();
                    days.push(days_staked);
                    self.data::<Data>()
                        .nft_staking_days
                        .insert(&(caller, item.clone()), &days);
                    // Step 5 - update `request_unstaking_time` to 0
                    self.data::<Data>()
                        .request_unstaking_time
                        .insert(&(caller, item.clone()), &0);

                    if self.data::<Data>().pending_unstaking_list.count(caller) == 0 {
                        self.data::<Data>().staked_accounts.remove_value(1, &caller);
                    }

                    // TODO: emit_event
                } else {
                    return Err(StakingError::FailedToCalculateTimeRequstUnstake);
                }
            } else {
                return Err(StakingError::FailedToCalculateTimeRequstUnstake);
            }
        }
        Ok(())
    }

    default fn get_staked_item(&self, account: AccountId, item: Id) -> Vec<u64> {
        self.data::<Data>()
            .nft_staking_days
            .get((account, item))
            .unwrap_or_default()
    }

    default fn claim_rewards(&mut self) -> Result<(), StakingError> {
        let caller = Self::env().caller();
        let is_claimed = self.data::<Data>().is_claimed.get(&caller);

        // Check if the claim exist and must be false
        if let Some(claimed) = is_claimed {
            if claimed {
                return Err(StakingError::ClaimMustBeFalse);
            } else {
                // self.data::<Data>().is_claimed.insert(&caller, &true); // Can only claim once



                Ok(())
            }
        } else {
            return Err(StakingError::ClaimMustBeFalse);
        }
    }

    default fn set_claimed_status(&mut self, staker: AccountId) -> Result<(), StakingError> {
        self.data::<Data>().is_claimed.insert(&staker, &false); // Can only claim once
        Ok(())
    }

    // Get User NFT staked in the contract
    default fn get_total_staked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Data>().staking_list.count(account) as u64;
    }

    // Get User NFT staked in the contract
    default fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Data>().pending_unstaking_list.count(account) as u64;
    }
}

pub trait Internal {
    fn get_request_unstake_time(&self, account: AccountId, token_id: Id) -> u64;
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn get_request_unstake_time(&self, account: AccountId, token_id: Id) -> u64 {
        self.data::<Data>()
            .request_unstaking_time
            .get((account, token_id))
            .unwrap_or_default()
    }
}
