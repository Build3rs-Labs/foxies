#![cfg_attr(not(feature = "std"), no_std, no_main)]

//const DAYS:u64 = 86400000; // Milliseconds in a day
const DAYS:u64 = 120000; // Milliseconds in a day (Testnet dummy)

#[ink::contract]
mod staking {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

    use crate::DAYS;

    use ink::storage::Mapping;

    use ink::prelude::vec;

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment
    };

    use ink::contract_ref;

    use random::Source;

    use psp34::Id;

    use psp34::PSP34;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StakingError {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        /// Thrown when account hasn't staked any chicken
        ChickenNotStaked,
        /// Thrown when a given Id doesn't exist in chickens NFT collection
        TokenNotExists,
        /// Thrown when a given Id isn't owned by account in chickens NFT collection
        TokenNotOwnedByCaller,
        /// Thrown contract doesn't have the allowance to transfer chicken NFT from account
        AllowanceInexistent,
        /// Thrown when an account has exceeded max chicken NFT stakes of 5.
        ExceededMaxStakes,
        /// Thrown when transfer from user to staking contract fails
        TransferFailed,
        /// Thrown when a user fails to stake
        FailedUnstake,
        /// Thrown when only the manager is allowed to call the method
        OnlyOwnerAllowed,
        /// Thrown when an account doesn't hold a fox NFT
        NotAFoxHolder,
        /// Thrown when an account hasn't staked any NFT
        HasNotStaked,
        /// Thrown when a failure happens when trying to claim AZERO from vault
        UnableToClaimAzero,
        /// Thrown when a mint fails
        MintFailed
    }

    #[ink(storage)]
    pub struct Staking {
        // Contract address of factory contract
        factory: Option<AccountId>,
        // Contract address of foxes NFT collection
        foxes: Option<AccountId>,
        // Contract address of chickens NFT collection
        chickens: Option<AccountId>,
        // A map of index of staked tokens and chickens NFT Ids of account
        // in order of addition to index
        staked_chickens: Mapping<(AccountId, u128), u128>,
        // A map of number of chickens staked by a given account
        number_of_chickens_staked: Mapping<AccountId, u128>,
        // UNIX timestamp in milliseconds from last time account
        // initiated a chicken staking schedule
        last_chickens_stake_time: Mapping<AccountId, u64>,
        // A map of index of staked tokens and foxes NFT Ids of account
        // in order of addition to index
        staked_foxes: Mapping<(AccountId, u128), u128>,
        // A map of number of foxes staked by a given account
        number_of_foxes_staked: Mapping<AccountId, u128>,
        // UNIX timestamp in milliseconds from last time account
        // initiated a fox staking schedule
        last_foxes_stake_time: Mapping<AccountId, u64>,
        // Amount of azero mintable per chicken per day
        daily_azero_per_chicken: u128,
        // Maximum number of azero rewards that can be generated per account
        // regardless of duration
        cap_per_account: u128,
        // Address of the manager
        owner: Option<AccountId>,
        // Get address of person staking a fox
        fox_staked_by: Mapping<u128, AccountId>,
        // Get address of fox who last stole all azero from a given account
        azero_last_stolen_by: Mapping<AccountId, Option<AccountId>>,
        // Get amount of azero stolen from a given account
        azero_last_stolen_amount: Mapping<AccountId, Option<Vec<Balance>>>,
        // Get the last steal made by a fox
        last_steal: Mapping<AccountId, Vec<Balance>>,
        // Get last time Azero was stolen by a fox
        azero_last_stolen_time: Mapping<AccountId, u128>,
        // Get Azero claimed
        azero_claimed: Balance,
        // Randomness seed
        seed: u64
    }

    impl Staking {
        #[ink(constructor)]
        pub fn new(factory:AccountId, foxes: AccountId, chickens: AccountId, daily_azero_per_chicken: u128, cap_per_account: u128) -> Self {
            let owner = Self::env().caller();
            let timestamp = Self::env().block_timestamp();
            Self {
                factory: Some(factory),
                chickens: Some(chickens),
                foxes: Some(foxes),
                owner: Some(owner),
                staked_foxes: Default::default(),
                number_of_foxes_staked: Default::default(),
                last_foxes_stake_time: Default::default(),
                staked_chickens: Default::default(),
                daily_azero_per_chicken,
                cap_per_account,
                last_chickens_stake_time: Default::default(),
                number_of_chickens_staked: Default::default(),
                fox_staked_by: Default::default(),
                azero_last_stolen_by: Default::default(),
                last_steal: Default::default(),
                azero_last_stolen_time: Default::default(),
                azero_claimed: 0,
                azero_last_stolen_amount: Default::default(),
                seed: timestamp
            }
        }
        
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            Self::env().account_id()
        }

        #[inline]
        pub fn update_seed(&mut self, adder: u64) {
            self.seed += adder;
        }

        // Stake a chicken of given Id from chickens NFT collection
        #[ink(message)]
        pub fn stake_chicken(&mut self, id: u128) -> Result<(), StakingError> {

            self.update_seed(1);

            let caller = self.env().caller(); // Caller address

            // Total NFTs staked by account
            let number_of_chickens_staked = self.number_of_chickens_staked.get(caller).unwrap_or(0);

            if number_of_chickens_staked == 5 {
                // Must not stake more than 5 chicken NFTs
                return Err(StakingError::ExceededMaxStakes);
            }

            // Ref {} of chickens NFT

            let mut nft: contract_ref!(PSP34) = self.chickens.unwrap().into();

            if let Some(ref owner) = nft.owner_of(Id::U128(id)) {
                // If owner exists for Id
                if caller != *owner {
                    // Caller must be owner
                    return Err(StakingError::TokenNotOwnedByCaller);
                }
                let allowance = nft.allowance(caller, Self::env().account_id(), Some(Id::U128(id)));
                // Make sure allowance exists to allow for transfer of NFTs from
                // account to staking contract
                if allowance == false {
                    return Err(StakingError::AllowanceInexistent);
                }
            }
            else {
                // Token Id doesn't exist
                return Err(StakingError::TokenNotExists);
            }

            // Get last time caller staked NFTs
            let last_chickens_stake_time = self.last_chickens_stake_time.get(caller).unwrap_or(0);

            if last_chickens_stake_time == 0 {
                // Set to UNIX block timestamp if not initially in any staking scheme
                self.last_chickens_stake_time.insert(caller, &self.env().block_timestamp());
            }

            // Get number of stakes and use as index for keying Ids
            let index = number_of_chickens_staked;

            // Insert staked chicken Id to index
            self.staked_chickens.insert((caller, index), &id);

            // Insert number of stakes +1 to 'account'
            self.number_of_chickens_staked.insert(caller, &(number_of_chickens_staked + 1));

            // Transfer token from caller to staking contract
            if nft.transfer_from(caller, Self::env().account_id(), Id::U128(id), vec![]).is_err() {
                return Err(StakingError::TransferFailed);
            }

            Ok(())

        }

        // Get list of chicken NFT Ids staked by 'account'
        #[ink(message)]
        pub fn get_staked_chickens(&self, account: AccountId) -> Vec<u128> {

            let mut vector = vec![]; // Initialize empty vector

            // Get number of stakes for use as iterator
            let number_of_chickens_staked = self.number_of_chickens_staked.get(account).unwrap_or(0);

            for nfts in 0..number_of_chickens_staked {

                // Loops indexes to get staked chicken Ids
                let nft_id = self.staked_chickens.get((account, u128::from(nfts))).unwrap_or(0);

                // Push NFT Id to vector
                vector.push(nft_id);
                
            }

            // Return vector
            vector

        }

        // Get list of foxes NFT Ids staked by 'account'
        #[ink(message)]
        pub fn get_staked_foxes(&self, account: AccountId) -> Vec<u128> {

            let mut vector = vec![]; // Initialize empty vector

            // Get number of stakes for use as iterator
            let number_of_foxes_staked = self.number_of_foxes_staked.get(account).unwrap_or(0);

            for nfts in 0..number_of_foxes_staked {

                // Loops indexes to get staked foxes Ids
                let nft_id = self.staked_foxes.get((account, u128::from(nfts))).unwrap_or(0);

                // Push NFT Id to vector
                vector.push(nft_id);
                
            }

            // Return vector
            vector

        }

        // Get address of fox who last stole all Azero from a given account
        #[ink(message)]
        pub fn get_last_fox_for_stolen_azero(&self, account: AccountId) -> Option<AccountId> {
            self.azero_last_stolen_by.get(account).unwrap_or(None)
        }

        // Get amount of Azero stolen from a given account
        #[ink(message)]
        pub fn get_last_amount_for_stolen_azero(&self, account: AccountId) -> Option<Vec<Balance>> {
            self.azero_last_stolen_amount.get(account).unwrap_or(None)
        }

        // Get the last time Azero was stolen from chicken
        #[ink(message)]
        pub fn get_last_time_for_stolen_azero(&self, account: AccountId) -> u128 {
            self.azero_last_stolen_time.get(account).unwrap_or(0)
        }

        // Get last steal by fox
        #[ink(message)]
        pub fn get_last_steal(&self, account: AccountId) -> Vec<Balance> {
            self.last_steal.get(account).unwrap_or(vec![0, 0])
        }

        // Get number of potentially claimable Azero by chicken staker
        #[ink(message)]
        pub fn get_claimable_azero(&self, account: AccountId) -> u128 {
            
            // Get count of stakes
            let number_of_chickens_staked = self.number_of_chickens_staked.get(account).unwrap_or(0);

            // Get last time staking program scheme was initiated
            let last_chickens_stake_time = self.last_chickens_stake_time.get(account).unwrap_or(0);

            // Get the number of milliseconds past since staking initiation
            let time_past = self.env().block_timestamp() - last_chickens_stake_time;

            let mut _claimable = 0;

            let days_past:u128 = (time_past / DAYS).try_into().unwrap();
            // Number of days past -> divide difference by seconds in a day

            _claimable = days_past * self.daily_azero_per_chicken * number_of_chickens_staked;
            // Get claimable by multiplying days past by daily azero supposedly earned by 
            // Number of chickens staked

            if _claimable >= self.cap_per_account {
                // Claimable rewards must not exceed earning cap per account
                _claimable = self.cap_per_account
            }

            if _claimable >= self.env().balance() {
                _claimable = self.env().balance();
            }

            _claimable
            
        }

        #[inline]
        pub fn random_int_from_range(&self, from: u64, to: u64) -> u64 {
            let mut source = random::default(self.env().block_timestamp());
            let rand_int:u64 = source.read::<u64>() % to + from;
            rand_int
        }

        // Unstake chickens staked by caller
        #[ink(message)]
        pub fn unstake_chickens(&mut self) -> Result<(), StakingError> {

            let caller = self.env().caller(); // caller

            let claimable = self.get_claimable_azero(caller); // Get claimable azero

            // Get number of chicken NFTs staked
            let number_of_chickens_staked = self.number_of_chickens_staked.get(caller).unwrap_or(0);

            if number_of_chickens_staked == 0 {
                // Make sure account has staked
                return Err(StakingError::HasNotStaked);
            }

            // Ref {} of chickens NFT contract
            let mut _nft: contract_ref!(PSP34) = self.chickens.unwrap().into();

            if claimable > 0 { // If there are Azero claimable
                
                let random_number = self.random_int_from_range(1, 2);

                let zero_account = AccountId::from([0u8; 32]); // Zero account

                // Instance A -> 1 (not stolen by fox)
                // Instance B -> 2 (stolen by fox)

                if random_number == 1 { // Instance A
                    // Owner takes 80% of the rewards
                    let amount_for_pool = (20 * claimable) / 100;
                    let amount_for_account = (80 * claimable) / 100;

                    // Mint 80% to caller
                    let _ = self.mint_and_transfer_azero_to_account(caller, amount_for_account);

                    self.azero_claimed += amount_for_account;

                    // Mint 20% to fox vault
                    let _ = self.mint_and_transfer_azero_to_account(Self::env().account_id(), amount_for_pool);

                    self.update_seed(1);
                }
                else { // Instance B
                    // Get random fox by calling factory
                    let random_fox = self.call_factory_for_random_fox_holder();
                    if random_fox == zero_account {
                        // If no fox exists, mint all to fox vault
                        let _ = self.mint_and_transfer_azero_to_account(Self::env().account_id(), claimable);
                    }
                    else {
                        // If a fox is returned, mint all to selected fox
                        let _ = self.mint_and_transfer_azero_to_account(random_fox, claimable);
                        self.azero_claimed += claimable;
                        self.last_steal.insert(random_fox, &vec![self.env().block_timestamp() as u128, claimable]);
                        self.azero_last_stolen_by.insert(caller, &Some(random_fox));
                        self.azero_last_stolen_amount.insert(caller, &Some(vec![self.env().block_timestamp() as u128, claimable]));
                        self.azero_last_stolen_time.insert(caller, &(self.env().block_timestamp() as u128));
                    }

                    self.update_seed(1);
                }

            }

            // Loop through NFTs staked and transfer them back to owner

            for nfts in 0..number_of_chickens_staked {

                let nft_id = self.staked_chickens.get((caller, u128::from(nfts))).unwrap_or(0);

                if _nft.transfer(caller, Id::U128(nft_id), vec![]).is_err() {
                    return Err(StakingError::FailedUnstake);
                }

                // Remove Id from staked NFTs of caller at incremental index
                self.staked_chickens.remove((caller, u128::from(nfts)));

            }

            // Reinitialize last stake time and number of stakes to 0
            self.last_chickens_stake_time.insert(caller, &0);
            self.number_of_chickens_staked.insert(caller, &0);

            Ok(())

        }

        // Cross-contract call to factory to get a random fox holder
        #[inline]
        pub fn call_factory_for_random_fox_holder(&self) -> AccountId {

            let ( mut _random_fox_holder, mut _nft_id ) = build_call::<DefaultEnvironment>()
            .call(self.factory.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "pick_random_fox_holder_with_rarity"
            ))))
            .returns::<(AccountId, u128)>()
            .try_invoke().unwrap().unwrap();

            if _random_fox_holder == self.env().account_id() {
                let fox_staked_by = self.fox_staked_by.get(_nft_id).unwrap_or(AccountId::from([0u8; 32]));
                if fox_staked_by == AccountId::from([0u8; 32]) {
                    _random_fox_holder = fox_staked_by;
                }
                else {
                    _random_fox_holder = _random_fox_holder;
                }
            }
            
            _random_fox_holder

        }

        // Get Azero amount to Azero claimed
        #[ink(message)]
        pub fn get_azero_claimed(&mut self) -> Balance {
            self.azero_claimed
        }

        // Cross-contract call to factory to get a random fox holder
        #[inline]
        pub fn call_factory_for_fox_rarity(&self, id: u128) -> u128 {

            let fox_rarity = build_call::<DefaultEnvironment>()
            .call(self.factory.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "get_fox_rarity"
            ))).push_arg(id))
            .returns::<u128>()
            .try_invoke().unwrap().unwrap();

            fox_rarity

        }

        // Get amount of azero in foxes vault
        #[ink(message)]
        pub fn get_azero_in_pool(&self) -> Balance {
            self.env().balance()
        }

        // Get claimable amount per foxes based on amount of azero in foxes vault
        #[inline]
        pub fn get_base_claim_per_fox(&self) -> Balance {
            let foxes: contract_ref!(PSP34) = self.foxes.unwrap().into(); // Ref {} of foxes contract
            // Evenly distribute Azero amongst the number of fox NFT holders
            if self.env().balance() > foxes.total_supply() {
                self.env().balance() / foxes.total_supply()
            }
            else {
                foxes.total_supply()
            }
        }

        // Determine claimable azero for fox staker
        #[ink(message)]
        pub fn get_claimable_for_fox(&self, account: AccountId) -> Balance {

            // Get number of staked foxes
            let number_of_foxes_staked = self.number_of_foxes_staked.get(account).unwrap_or(0);

            // Used to count claimable tokens
            let mut _claimable = 0;

            // Last time in UNIX timestamp staking schedule was initiated
            let last_foxes_stake_time = self.last_foxes_stake_time.get(account).unwrap_or(0);

            // Get the number of milliseconds past since staking initiation
            let time_past = self.env().block_timestamp() - last_foxes_stake_time;

            // Number of days past -> divide difference by seconds in a day
            let days_past:u128 = (time_past / DAYS).try_into().unwrap();

            // Loop through foxes
            for nfts in 0..number_of_foxes_staked {
                let nft_id = self.staked_foxes.get((account, u128::from(nfts))).unwrap_or(0);
                let rarity = self.call_factory_for_fox_rarity(nft_id);

                // Increment claimable as base claim per fox by rarity of fox by days past by 1000

                _claimable += self.get_base_claim_per_fox() * rarity * days_past * 1000;
            }

            if _claimable >= self.get_azero_in_pool() {
                _claimable = self.get_azero_in_pool();
            }
            
            // Return potentially claimable azero
            _claimable

        }

        // Stake a fox of given Id from foxes NFT collection
        #[ink(message)]
        pub fn stake_fox(&mut self, id: u128) -> Result<(), StakingError> {

            self.update_seed(2);

            let caller = self.env().caller(); // Caller address

            // Total NFTs staked by account
            let number_of_foxes_staked = self.number_of_foxes_staked.get(caller).unwrap_or(0);

            if number_of_foxes_staked == 5 {
                // Must not stake more than 5 foxes NFTs
                return Err(StakingError::ExceededMaxStakes);
            }

            // Ref {} of foxes NFT

            let mut nft: contract_ref!(PSP34) = self.foxes.unwrap().into();

            if let Some(ref owner) = nft.owner_of(Id::U128(id)) {
                // If owner exists for Id
                if caller != *owner {
                    // Caller must be owner
                    return Err(StakingError::TokenNotOwnedByCaller);
                }
                let allowance = nft.allowance(caller, Self::env().account_id(), Some(Id::U128(id)));
                // Make sure allowance exists to allow for transfer of NFTs from
                // account to staking contract
                if allowance == false {
                    return Err(StakingError::AllowanceInexistent);
                }
            }
            else {
                // Token Id doesn't exist
                return Err(StakingError::TokenNotExists);
            }

            // Get last time caller staked NFTs
            let last_foxes_stake_time = self.last_foxes_stake_time.get(caller).unwrap_or(0);

            if last_foxes_stake_time == 0 {
                // Set to UNIX block timestamp if not initially in any staking scheme
                self.last_foxes_stake_time.insert(caller, &self.env().block_timestamp());
            }

            // Get number of stakes and use as index for keying Ids
            let index = number_of_foxes_staked;

            // Insert staked fox Id to index
            self.staked_foxes.insert((caller, index), &id);

            self.fox_staked_by.insert(id, &caller);

            // Insert number of stakes +1 to 'account'
            self.number_of_foxes_staked.insert(caller, &(number_of_foxes_staked + 1));

            // Transfer token from caller to staking contract
            if nft.transfer_from(caller, Self::env().account_id(), Id::U128(id), vec![]).is_err() {
                return Err(StakingError::TransferFailed);
            }

            Ok(())

        }

        // Unstake foxes and send rewards to fox staker: Max stakes is 5 foxes
        #[ink(message)]
        pub fn unstake_foxes(&mut self) -> Result<(), StakingError> {

            self.update_seed(2);

            let account = self.env().caller(); // caller

            let claimable = self.get_claimable_for_fox(account); // Get claimable Azero

            // Get number of foxes NFTs staked
            let number_of_foxes_staked = self.number_of_foxes_staked.get(account).unwrap_or(0);

            if number_of_foxes_staked == 0 {
                // Make sure account has staked
                return Err(StakingError::HasNotStaked);
            }

            // Ref {} foxes NFT contract
            let mut _foxes: contract_ref!(PSP34) = self.foxes.unwrap().into();

            for nfts in 0..number_of_foxes_staked {

                let nft_id = self.staked_foxes.get((account, u128::from(nfts))).unwrap_or(0);

                if _foxes.transfer(account, Id::U128(nft_id), vec![]).is_err() {
                    return Err(StakingError::FailedUnstake);
                }

                // Remove Id from staked NFTs of caller at incremental index
                self.staked_foxes.remove((account, u128::from(nfts)));

                self.fox_staked_by.remove(u128::from(nfts));

            }

            if claimable > 0 {
                // Transfer claimable azero to fox holder
                if self.env().transfer(account, claimable).is_err() {
                    return Err(StakingError::UnableToClaimAzero);
                }
                self.azero_claimed += claimable;
            }
            // Reinitialize last stake time and number of stakes to 0
            self.last_foxes_stake_time.insert(account, &0);
            self.number_of_foxes_staked.insert(account, &0);

            Ok(())
        }

        #[inline]
        pub fn mint_and_transfer_azero_to_account(&mut self, account: AccountId, amount: u128) -> Result<(), StakingError> {

            if account == self.env().account_id() {
                return Ok(());
            }
            if account != Self::env().account_id() {
                if self.env().transfer(account, amount).is_err() {
                    return Err(StakingError::TransferFailed);
                }
            }

            Ok(())

        }
        
    }
}