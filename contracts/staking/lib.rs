#![cfg_attr(not(feature = "std"), no_std, no_main)]

const DAYS:u64 = 86400000; // Milliseconds in a day

#[ink::contract]
mod staking {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

    use crate::DAYS;

    use ink::storage::Mapping;

    use ink::prelude::vec;

    use ink::{
        env::{
            call::{build_call, Call, ExecutionInput, Selector},
            DefaultEnvironment, Error as InkEnvError,
            CallFlags
        },
        LangError,
    };

    use ink::contract_ref;

    use random::Source;

    use foxes::Id;

    use foxes::PSP34;

    use eggs::PSP22;

    use eggs::PSP22Mintable;

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
        /// Thrown when there are no claimable eggs for chicken staker
        NoClaimableRewards,
        /// Thrown when a user fails to stake
        FailedUnstake,
        /// Thrown when only the manager is allowed to call the method
        OnlyOwnerAllowed,
        /// Thrown when an account doesn't hold a fox NFT
        NotAFoxHolder,
        /// Thrown when an account hasn't staked any NFT
        HasNotStaked,
        /// Thrown when a fox holder doesn't have claimable eggs
        NoClaimableEggs,
        /// Thrown when a failure happens when trying to claim eggs from vault
        UnableToClaimEggs
    }

    #[ink(storage)]
    pub struct Staking {
        // Contract address of factory contract
        factory: Option<AccountId>,
        // Contract address of eggs token
        eggs: Option<AccountId>,
        // Contract address of foxes NFT collection
        foxes: Option<AccountId>,
        // Contract address of chickens NFT collection
        chickens: Option<AccountId>,
        // A map of index of staked tokens and NFT Ids of
        // in order of addition to index
        staked_chickens: Mapping<(AccountId, u128), u128>,
        // A map of number of chickens staked by a given account
        number_of_stakes: Mapping<AccountId, u128>,
        // UNIX timestamp in milliseconds from last time accout
        // initiated a chicken staking schedule
        last_stake_time: Mapping<AccountId, u64>,
        // Amount of eggs claimable per chicken per day
        daily_eggs_per_chicken: u128,
        // Maximum number of egg rewards that can be generated per account
        // regardless of duration
        cap_per_account: u128,
        // UNIX timestamp in milliseconds from last time eggs were
        // claimed from vault by a fox holder
        eggs_last_claimed: Mapping<AccountId, u64>,
        // Address of the manager
        owner: Option<AccountId>
    }

    impl Staking {
        #[ink(constructor)]
        pub fn new(factory:AccountId, foxes: AccountId, chickens: AccountId, daily_eggs_per_chicken: u128, cap_per_account: u128) -> Self {
            let owner = Self::env().caller();
            Self {
                factory: Some(factory),
                eggs: None,
                chickens: Some(chickens),
                foxes: Some(foxes),
                owner: Some(owner),
                staked_chickens: Default::default(),
                daily_eggs_per_chicken,
                cap_per_account,
                last_stake_time: Default::default(),
                number_of_stakes: Default::default(),
                eggs_last_claimed: Default::default()
            }
        }
        
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            Self::env().account_id()
        }

        // Set eggs contract address: Only manager can call this method
        #[ink(message)]
        pub fn set_eggs_address(&mut self, address: AccountId) -> Result<(), StakingError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(StakingError::OnlyOwnerAllowed);
            }
            self.eggs = Some(address);
            Ok(())
        }

        // Stake a chicken of given Id from chickens NFT collection
        #[ink(message)]
        pub fn stake_chicken(&mut self, id: u128) -> Result<(), StakingError> {

            let caller = self.env().caller(); // Caller address

            // Total NFTs staked by account
            let number_of_stakes = self.number_of_stakes.get(caller).unwrap_or(0);

            if number_of_stakes == 5 {
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
            let last_stake_time = self.last_stake_time.get(caller).unwrap_or(0);

            if last_stake_time == 0 {
                // Set to UNIX block timestamp if not initially in any staking scheme
                self.last_stake_time.insert(caller, &self.env().block_timestamp());
            }

            // Get number of stakes and use as index for keying Ids
            let index = number_of_stakes;

            // Insert staked chicken Id to index
            self.staked_chickens.insert((caller, index), &id);

            // Insert number of stakes +1 to 'account'
            self.number_of_stakes.insert(caller, &(number_of_stakes + 1));

            // Transfer token from caller to staking contract
            if nft.transfer_from(caller, Self::env().account_id(), Id::U128(id), vec![]).is_err() {
                return Err(StakingError::TransferFailed);
            }

            Ok(())

        }

        // Get list of NFT Ids staked by 'account'
        #[ink(message)]
        pub fn get_staked_tokens(&self, account: AccountId) -> Vec<u128> {

            let mut vector = vec![]; // Initialize empty vector

            // Get number of stakes for use as iterator
            let number_of_stakes = self.number_of_stakes.get(account).unwrap_or(0);

            for nfts in 0..number_of_stakes {

                // Loops indexes to get staked chicken Ids
                let nft_id = self.staked_chickens.get((account, u128::from(nfts))).unwrap_or(0);

                // Push NFT Id to vector
                vector.push(nft_id);
                
            }

            // Return vector
            vector

        }

        // Get number of potentially claimable eggs by chicken staker
        #[ink(message)]
        pub fn get_claimable_eggs(&self, account: AccountId) -> u128 {
            
            // Get count of stakes
            let number_of_stakes = self.number_of_stakes.get(account).unwrap_or(0);

            // Get last time staking program scheme was initiated
            let last_stake_time = self.last_stake_time.get(account).unwrap_or(0);

            // Get the number of milliseconds past since staking initiation
            let time_past = self.env().block_timestamp() - last_stake_time;

            let mut claimable = 0;

            let days_past:u128 = (time_past / DAYS).try_into().unwrap();
            // Number of days past -> divide difference by seconds in a day

            claimable = days_past * self.daily_eggs_per_chicken * number_of_stakes;
            // Get claimable by multiplying days past by daily eggs supposedly earned by 
            // Number of chickens staked

            if claimable >= self.cap_per_account {
                // Claimable rewards must not exceed earning cap per account
                claimable = self.cap_per_account
            }

            claimable
            
        }

        // Unstake chickens staked by caller
        #[ink(message)]
        pub fn unstake_chickens(&mut self) -> Result<(), StakingError> {

            let caller = self.env().caller(); // caller

            let claimable = self.get_claimable_eggs(caller); // Get claimable eggs

            if claimable == 0 {
                // Err(()) when no claimable rewards
                return Err(StakingError::NoClaimableRewards);
            }

            // Get number of chicken NFTs staked
            let number_of_stakes = self.number_of_stakes.get(caller).unwrap_or(0);

            if number_of_stakes == 0 {
                // Make sure account has staked
                return Err(StakingError::HasNotStaked);
            }

            // Ref {} of eggs contract
            let mut eggs: contract_ref!(PSP22Mintable) = self.eggs.unwrap().into();

            // Ref {} of chickens NFT contract
            let mut nft: contract_ref!(PSP34) = self.chickens.unwrap().into();

            let mut source = random::default(self.env().block_timestamp());
            let random_number = source.read::<u64>() % 2 + 1; // Random between 1 and 2

            let zero_account = AccountId::from([0u8; 32]); // Zero account

            // Instance A -> 1 (not stolen by fox)
            // Instance B -> 2 (stolen by fox)

            if random_number == 1 { // Instance A
                // Owner takes 80% of the rewards
                //
                let amount_for_pool = (20 * claimable) / 100;
                let amount_for_account = (80 * claimable) / 100;

                // Mint 80% to caller
                let _ = eggs.mint(caller, amount_for_account);
                // Mint 20% to fox eggs vault
                let _ = eggs.mint(Self::env().account_id(), amount_for_pool);
            }
            else { // Instance B
                // Get random fox by calling factory
                let random_fox = self.call_factory_for_random_fox_holder();
                if random_fox == zero_account {
                    // If no fox exists, mint all to fox eggs vault
                    let _ = eggs.mint(Self::env().account_id(), claimable);
                }
                else {
                    // If a fox is returned, mint all to selected fox
                    let _ = eggs.mint(random_fox, claimable);
                }
            }

            // Loop through NFTs staked and transfer them back to owner

            for nfts in 0..number_of_stakes {

                let nft_id = self.staked_chickens.get((caller, u128::from(nfts))).unwrap_or(0);

                if nft.transfer(caller, Id::U128(nft_id), vec![]).is_err() {
                    return Err(StakingError::FailedUnstake);
                }

                // Remove Id from staked NFTs of caller at incremental index
                self.staked_chickens.remove((caller, u128::from(nfts)));

            }

            // Reinitialize last stake time and number of stakes to 0
            self.last_stake_time.insert(caller, &0);
            self.number_of_stakes.insert(caller, &0);

            Ok(())

        }

        // Cross-contract call to factory to get a random fox holder
        #[inline]
        pub fn call_factory_for_random_fox_holder(&self) -> AccountId {

            let random_fox_holder = build_call::<DefaultEnvironment>()
            .call(self.factory.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "pick_random_fox_holder_with_rarity"
            ))))
            .returns::<AccountId>()
            .try_invoke().unwrap().unwrap();

            random_fox_holder

        }

        // Get amount of eggs in foxes vault
        #[ink(message)]
        pub fn get_eggs_in_pool(&self) -> Balance {
            let eggs: contract_ref!(PSP22) = self.eggs.unwrap().into();
            eggs.balance_of(Self::env().account_id())
        }

        // Get claimable amount per foxes based on amount of eggs in foxes vault
        #[inline]
        pub fn get_claim_per_fox(&self) -> Balance {
            let eggs: contract_ref!(PSP22) = self.eggs.unwrap().into(); // Ref {} of eggs contract
            let foxes: contract_ref!(PSP34) = self.foxes.unwrap().into(); // Ref {} of foxes contract

            // Evenly distribute eggs amongst the number of fox NFT holders
            eggs.balance_of(Self::env().account_id()) / foxes.total_supply()
        }

        // Determine claimable eggs for fox
        #[ink(message)]
        pub fn get_claimable_for_fox(&self, account: AccountId) -> Balance {
            let foxes: contract_ref!(PSP34) = self.foxes.unwrap().into();
            let holds_fox = foxes.balance_of(account);
            if holds_fox == 0 {
                // User doesn't hold foxes NFT
                return 0;
            }
            let eggs_last_claimed = self.eggs_last_claimed.get(account).unwrap_or(0);
            if (self.env().block_timestamp() - eggs_last_claimed) >= DAYS {
                // Must have not claimed within the past day
                return self.get_claim_per_fox();
            }
            else {
                // Returns 0 if already claimed within the last day
                return 0;
            }
        }

        // Claim eggs for caller (fox holder)
        #[ink(message)]
        pub fn claim_eggs(&mut self) -> Result<(), StakingError> {

            let account = self.env().caller(); // caller

            // Ref {} foxes NFT contract
            let foxes: contract_ref!(PSP34) = self.foxes.unwrap().into();

            // Ref {} eggs contract
            let mut eggs: contract_ref!(PSP22) = self.foxes.unwrap().into();

            // Balance of caller for foxes NFT
            let holds_fox = foxes.balance_of(account);
            
            if holds_fox == 0 {
                // Must be a foxes NFT holder
                return Err(StakingError::NotAFoxHolder);
            }

            let claimable = self.get_claimable_for_fox(account);

            if claimable == 0 {
                // No eggs to claim
                return Err(StakingError::NoClaimableEggs);
            }

            if eggs.transfer(account, claimable, vec![]).is_err() {
                // Transfer claimable eggs to fox holder
                return Err(StakingError::UnableToClaimEggs);
            }

            // Insert UNIX timestamp for current block as last claimed for eggs
            self.eggs_last_claimed.insert(account, &self.env().block_timestamp());

            Ok(())
        }

        // Get the balance of an egg holder
        #[ink(message)]
        pub fn get_eggs_balance(&mut self, account: AccountId) -> Balance {

            let mut eggs: contract_ref!(PSP22) = self.eggs.unwrap().into(); // Ref {} eggs contract

            // Balance of account $EGGS
            eggs.balance_of(account)

        }
        
    }
}