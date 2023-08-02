use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use openbrush::{
    contracts::psp34::{Id, PSP34Error},
    storage::{MultiMapping, TypeGuard, ValueGuard},
    traits::{AccountId, Balance, String, ZERO_ADDRESS},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub admin_address: AccountId,
    pub nft_contract_address: AccountId,
    pub eggs_token_address: AccountId,
    pub staked_accounts: MultiMapping<u8, AccountId, ValueGuard<u8>>, // 0 is staked status, 1 is request unstake status
    pub total_staked: u64,
    pub staking_list: MultiMapping<AccountId, Id, ValueGuard<AccountId>>,
    pub pending_unstaking_list: MultiMapping<AccountId, Id, ValueGuard<AccountId>>,
    pub limit_unstaking_time: u64, // minutes
    pub amount_of_eggs_token_earn_per_day: Balance,
    pub is_claimed: Mapping<AccountId, bool>,
    pub staking_start_time: Mapping<(AccountId, Id), u64>,
    pub request_unstaking_time: Mapping<(AccountId, Id), u64>,
    pub unstaking_time: Mapping<(AccountId, Id), u64>,
    pub nft_staking_days: Mapping<(AccountId, Id), u64>,
    pub reward_pool: Balance,
    pub claimable_reward: Balance,
    pub _reserved: Option<()>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            admin_address: ZERO_ADDRESS.into(),
            nft_contract_address: ZERO_ADDRESS.into(),
            eggs_token_address: ZERO_ADDRESS.into(),
            staked_accounts: Default::default(),
            staking_list: Default::default(),
            total_staked: Default::default(),
            pending_unstaking_list: Default::default(),
            limit_unstaking_time: Default::default(),
            amount_of_eggs_token_earn_per_day: Default::default(),
            is_claimed: Default::default(),
            request_unstaking_time: Default::default(),
            staking_start_time: Default::default(),
            unstaking_time: Default::default(),
            reward_pool: Default::default(),
            nft_staking_days: Default::default(),
            claimable_reward: Default::default(),
            _reserved: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingError {
    NotTokenOwner,
    NotApproved,
    CannotTransfer,
    CannotFindTokenOwner,
    FailedToIncreaseTotalStaked,
    FailedToDescreaseTotalStaked,
    PSP34Error(PSP34Error),
    InvalidCaller,
    InvalidInput,
    InvalidTime,
    NotEnoughtTimeToRequestUnstake,
    FailedToCalculateTimeRequstUnstake,
    ClaimMustBeFalse,
    InvalidTotalStake,
    InvalidUserStake,
    InvalidRewardPool,
    NotEnoughBalance,
    FailToDecreaseClaimableReward,
    FailedToCalculateReward,
    CantStakeFoxesToken
}

impl StakingError {
    pub fn as_str(&self) -> String {
        match self {
            StakingError::CantStakeFoxesToken => String::from("CantStakeFoxesToken"),
            StakingError::NotTokenOwner => String::from("NotTokenOwner"),
            StakingError::NotApproved => String::from("NotApproved"),
            StakingError::CannotTransfer => String::from("CannotTransfer"),
            StakingError::CannotFindTokenOwner => String::from("CannotFindTokenOwner"),
            StakingError::InvalidInput => String::from("InvalidInput"),
            StakingError::InvalidTime => String::from("InvalidTime"),
            StakingError::ClaimMustBeFalse => String::from("ClaimMustBeFalse"),
            StakingError::InvalidTotalStake => String::from("InvalidTotalStake"),
            StakingError::InvalidUserStake => String::from("InvalidUserStake"),
            StakingError::InvalidRewardPool => String::from("InvalidRewardPool"),
            StakingError::NotEnoughBalance => String::from("NotEnoughBalance"),
            StakingError::FailToDecreaseClaimableReward => {
                String::from("FailToDecreaseClaimableReward")
            }
            StakingError::FailedToCalculateReward => String::from("FailedToCalculateReward"),
            StakingError::FailedToCalculateTimeRequstUnstake => {
                String::from("FailedToCalculateTimeRequstUnstake")
            }
            StakingError::InvalidCaller => String::from("InvalidCaller"),
            StakingError::NotEnoughtTimeToRequestUnstake => {
                String::from("NotEnoughtTimeToRequestUnstake")
            }
            StakingError::FailedToIncreaseTotalStaked => {
                String::from("FailedToIncreaseTotalStaked")
            }
            StakingError::FailedToDescreaseTotalStaked => {
                String::from("FailedToDescreaseTotalStaked")
            }
            StakingError::PSP34Error(_) => todo!(),
        }
    }
}

impl From<PSP34Error> for StakingError {
    fn from(error: PSP34Error) -> Self {
        StakingError::PSP34Error(error)
    }
}

pub struct RequestUnstakingTimeKey;

impl<'a> TypeGuard<'a> for RequestUnstakingTimeKey {
    type Type = &'a (&'a AccountId, &'a u64);
}
