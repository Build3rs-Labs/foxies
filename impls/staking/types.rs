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
    // Contract owner address
    pub admin_address: AccountId,
    // NFT contract address
    pub nft_contract_address: AccountId,
    // `$Eggs` token (i.e. psp22) address
    pub eggs_token_address: AccountId,
    // All chicken staked token count
    pub total_staked: u64,
    // Staking list mapping
    pub staking_list: MultiMapping<AccountId, Id, ValueGuard<AccountId>>,
    // pending unstaking list
    pub pending_unstaking_list: MultiMapping<AccountId, Id, ValueGuard<AccountId>>,
    // Total number of token staked by account
    pub total_staked_token_by_account: Mapping<AccountId, u64>,
    // Total number of token staked
    pub total_staked_token: Mapping<u64, (AccountId, Id)>,
    // unstaking time limit
    pub limit_unstaking_time: u64,
    // request unstaking time
    pub request_unstaking_time: Mapping<(AccountId, Id), u64, RequestUnstakingTimeKey>,
    // Earn `$Eggs` per day by each staked token
    pub amount_of_eggs_token_earn_per_day: Balance,
    pub is_claimed: Mapping<AccountId, bool>,
    pub _reserved: Option<()>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            admin_address: ZERO_ADDRESS.into(),
            nft_contract_address: ZERO_ADDRESS.into(),
            eggs_token_address: ZERO_ADDRESS.into(),
            staking_list: Default::default(),
            total_staked: Default::default(),
            pending_unstaking_list: Default::default(),
            total_staked_token_by_account: Mapping::default(),
            total_staked_token: Mapping::default(),
            limit_unstaking_time: Default::default(),
            amount_of_eggs_token_earn_per_day: Default::default(),
            is_claimed: Default::default(),
            request_unstaking_time: Default::default(),
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
    NotEnoughtTimeToRequestUnstake,
    FailedToCalculateTimeRequstUnstake,
}

impl StakingError {
    pub fn as_str(&self) -> String {
        match self {
            StakingError::NotTokenOwner => String::from("NotTokenOwner"),
            StakingError::NotApproved => String::from("NotApproved"),
            StakingError::CannotTransfer => String::from("CannotTransfer"),
            StakingError::CannotFindTokenOwner => String::from("CannotFindTokenOwner"),
            StakingError::InvalidInput => String::from("InvalidInput"),
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
