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
                        .insert(&caller, &vec![item.clone()]);

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
    default fn un_stake(&mut self, token_ids: Vec<Id>) -> Result<(), StakingError> {
        Ok(())
    }

    default fn get_staking_list_token(&self, account: AccountId) -> Vec<Id> {
        self.data::<Data>()
            .staking_list
            .get(&account)
            .unwrap_or_default()
    }

    default fn get_total_staked_chickens_by_account(&self, account: AccountId) -> u64 {
        self.data::<Data>()
            .total_staked_token_by_account
            .get(account)
            .unwrap_or_default()
    }
}
