use crate::impls::staking::types::Data;
pub use crate::traits::staking::Staking;
use ink::prelude::vec;
use ink::{env::CallFlags, prelude::vec::Vec};
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
                    // self.data::<Data>().staking_list.insert(caller, &item.clone());
                    self.data::<Data>()
                        .staking_list
                        // .insert(&caller, &vec![item.clone()]);
                        .insert(caller, item);

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

                        // TODO: emit event
                    }
                } else {
                    return Err(StakingError::CannotFindTokenOwner);
                }
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

                // Step 2 - Check staker
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

                // Step 4 - Add token to pending unstaking list
                let current_time = Self::env().block_timestamp();

                self.data::<Data>()
                    .request_unstaking_time
                    .insert(&(caller, item.clone()), &current_time);
                self.data::<Data>()
                    .pending_unstaking_list
                    .insert(caller, &item.clone());

                // TODO: emit_event
            } else {
                return Err(StakingError::CannotFindTokenOwner);
            }
        }

        if let Some(total_staked) = self.data::<Data>().total_staked.checked_sub(leng as u64) {
            self.data::<Data>().total_staked = total_staked;
            Ok(())
        } else {
            return Err(StakingError::FailedToDescreaseTotalStaked);
        }
    }

    default fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError> {
        // Step 1 - Check if the token is belong to caller
        let caller = Self::env().caller();
        let current_time = Self::env().block_timestamp();

        for item in token_ids.iter() {
            // Step 2 - Check request unstaked and time request unstaked
            // 1 min = 60000 milliseconds
            let request_unstake_time = self.get_request_unstake_time(caller, item.clone());

            if let Some(checked_mul_value) =
                self.data::<Data>().limit_unstaking_time.checked_add(60000)
            {
                if let Some(unstake_time) = request_unstake_time.checked_add(checked_mul_value) {
                    if unstake_time > current_time {
                        return Err(StakingError::NotEnoughtTimeToRequestUnstake);
                    }

                    // Step 3 - transfer token to caller if enough time
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
                    self.data::<Data>()
                        .request_unstaking_time
                        .insert(&(caller, item.clone()), &0);

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
