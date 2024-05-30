#![cfg_attr(not(feature = "std"), no_std, no_main)]

const AZERO_FOR_DIRECT_FOX_MINT:u128 = 100 * 10u128.pow(12); // 100 AZERO for defined fox mint

#[ink::contract]
mod factory {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

    use crate::AZERO_FOR_DIRECT_FOX_MINT;

    use random::Source;

    use ink::storage::Mapping;

    use ink::prelude::vec;

    use ink::contract_ref;

    use psp34::{
        PSP34Mintable,
        PSP34Error,
        PSP34,
        Id
    };

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum FactoryError {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        /// Returned when mint has failed (maybe total supply netted)
        FailedMint,
        /// Returned when claiming of minted NFTs has failed
        FailedClaim,
        /// Returned when caller is not owner for owner-only methods
        OnlyOwnerAllowed,
        /// Returned when amount sent is not valid payment
        InvalidMintPayment,
        /// Returned when a user has exceeded their max direct fox mints
        ExceededDirectFoxMintAllowance,
        /// Failed to send AZERO
        FailedAZEROTransfer,
        // Returned when mint type by admin is invalid
        InvalidMintType
    }

    #[ink(storage)]
    pub struct Factory {
        // A map of rarities and the IDs with same rarity value: Rarities are between 1 and 50
        rarities: Mapping<u128, Vec<u128>>,
        // A vec of all fox NFTs minted
        nfts: Vec<u128>,
        // A map of NFT IDs and their respective rarity values
        nfts_rarity: Mapping<u128, u128>,
        // Contract address of the Chickens NFT contract
        chickens_nft_address: Option<AccountId>,
        // Contract address of the Foxes NFT contract
        foxes_nft_address: Option<AccountId>,
        // Address of the manager
        owner: Option<AccountId>,
        // Represents if it was a fox or a chicken last minted by a given account. 0 for chicken, 1 for fox
        last_mint: Mapping<AccountId, Option<(u8, u128)>>,
        // Count of chickens minted
        chickens_minted: u128,
        // Count of direct fox mints done by user
        direct_fox_mints: Mapping<AccountId, u8>,
        // AZERO for direct fox mints: Mutable
        azero_for_direct_fox_mints: u128,
        // Account for minting fees to be sent to
        fees_account: Option<AccountId>,
        // Rewards to staking pool
        rewards_pool: Option<AccountId>,
        // A list of whitelisted callers
        whitelisted: Mapping<AccountId, bool>,
        // AZERO traded
        azero_traded: u128,
        // AZERO claimed
        azero_claimed: u128,
        // Randomness seed
        seed: u64,
        // Randomness oracle
        oracle: Option<AccountId>,
        // Allowed claim
        allowed_mint: Mapping<AccountId, bool>,
        // NFT to claim
        nft_to_claim: Mapping<AccountId, (u64, u64)>,
    }

    impl Factory {
        #[ink(constructor)]
        pub fn new(fees_account: AccountId, oracle: AccountId) -> Self {
            let caller = Self::env().caller();
            let timestamp = Self::env().block_timestamp();
            Self {
                rarities: Mapping::default(),
                nfts: vec![],
                nfts_rarity: Mapping::default(),
                chickens_nft_address: None,
                foxes_nft_address: None,
                owner: Some(caller),
                last_mint: Mapping::default(),
                chickens_minted: 0,
                direct_fox_mints: Mapping::default(),
                azero_for_direct_fox_mints: AZERO_FOR_DIRECT_FOX_MINT,
                fees_account: Some(fees_account),
                rewards_pool: None,
                whitelisted: Mapping::default(),
                azero_traded: 0,
                azero_claimed: 0,
                seed: timestamp,
                oracle: Some(oracle),
                allowed_mint: Mapping::default(),
                nft_to_claim: Mapping::default()
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

        // Set staking contract address: Only manager can call this method
        #[ink(message)]
        pub fn set_staking_address(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.rewards_pool = Some(address);
            self.whitelisted.insert(address, &true);
            Ok(())
        }

        // Set chickens NFT address: Only manager can call this method
        #[ink(message)]
        pub fn set_chickens_nft_address(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.chickens_nft_address = Some(address);
            self.whitelisted.insert(address, &true);
            Ok(())
        }

        // Set foxes NFT address: Only manager can call this method
        #[ink(message)]
        pub fn set_foxes_nft_address(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.foxes_nft_address = Some(address);
            self.whitelisted.insert(address, &true);
            Ok(())
        }

        // Make an address whitelisted: Only manager can call this method
        #[ink(message)]
        pub fn add_whitelisted(&mut self, address: AccountId, to_have:bool) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.whitelisted.insert(address, &to_have);
            Ok(())
        }

        // Make an address whitelisted: Only manager can call this method
        #[ink(message)]
        pub fn remove_whitelisted(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.whitelisted.insert(address, &false);
            Ok(())
        }

        // Set the amount of AZERO for direct fox mints: Only manager can call this method
        #[ink(message)]
        pub fn set_azero_for_direct_fox_mints(&mut self, amount: u128) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.azero_for_direct_fox_mints = amount;
            Ok(())
        }

        #[ink(message)]
        pub fn get_azero_for_direct_fox_mints(&mut self) -> Balance {
            self.azero_for_direct_fox_mints
        }

        #[ink(message)]
        pub fn get_platform_status(&mut self) -> (Balance, Balance, Balance, Balance, Balance) {

            let chickens_ca = self.chickens_nft_address;
            let foxes_ca = self.foxes_nft_address;
            let staking_ca = self.rewards_pool;

            let mut found = 0;

            match chickens_ca {
                Some(_value) => {
                    found += 1;
                },
                None => {},
            };
            match foxes_ca {
                Some(_value) => {
                    found += 1;
                },
                None => {},
            };
            match staking_ca {
                Some(_value) => {
                    found += 1;
                },
                None => {},
            };

            if found < 3 {
                return (0, 0, 0, 0, 0);
            }
            else {
                let chickens_nft: contract_ref!(PSP34) = chickens_ca.unwrap().into();
                let foxes_nft: contract_ref!(PSP34) = foxes_ca.unwrap().into();
                
                let foxes_minted = foxes_nft.total_supply();
                let chickens_minted = chickens_nft.total_supply();

                let total_minted = chickens_minted + foxes_minted;

                let azero_traded = self.azero_traded;

                let azero_claimed = build_call::<DefaultEnvironment>()
                .call(self.rewards_pool.unwrap())
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "get_azero_claimed"
                ))))
                .returns::<Balance>()
                .try_invoke().unwrap().unwrap();

                return (total_minted, foxes_minted, chickens_minted, azero_traded, azero_claimed);
            }
            
        }

        // Gets the amount of AZERO for random mints
        #[ink(message)]
        pub fn get_azero_for_random_mints(&mut self) -> Balance {

            let chickens_nft: contract_ref!(PSP34) = self.chickens_nft_address.unwrap().into();
            let foxes_nft: contract_ref!(PSP34) = self.foxes_nft_address.unwrap().into();

            let total_supply = chickens_nft.total_supply() + foxes_nft.total_supply();

            let denom = 10u128.pow(12);
            let price;

            if total_supply >= 0 && total_supply < 2000 {
                price = 5 * denom;
            }
            else if total_supply >= 2000 && total_supply < 5000 {
                price = 10 * denom;
            }
            else if total_supply >= 5000 && total_supply < 10000 {
                price = 15 * denom;
            }
            else if total_supply >= 10000 && total_supply < 12000 {
                price = 20 * denom;
            }
            else {
                price = 25 * denom;
            }
            price
        }

        #[ink(message)]
        pub fn is_allowed_mint(&self, account: AccountId) -> bool {
            self.allowed_mint.get(account).unwrap_or(true)
        }

        #[ink(message)]
        pub fn mint_by_admin(&mut self, mint_type: u8, account: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            if mint_type == 0 {
                // 0 for Chicken
                let mint = self.mint_chicken(account);
                if mint.is_err() {
                    return Err(FactoryError::FailedMint);
                }
                // Record last mint for account as chicken
                self.last_mint.insert(account, &Some((0, mint.unwrap())));
            }
            else if mint_type == 1 {
                // 1 for Fox
                let mint = self.mint_fox(account);
                if mint.is_err() {
                    return Err(FactoryError::FailedMint);
                }
                // Record last mint for account as fox
                self.last_mint.insert(account, &Some((1, mint.unwrap())));
            }
            else {
                return Err(FactoryError::InvalidMintType);
            }
            self.update_seed(3);
            Ok(())
        }

        #[ink(message)]
        pub fn claim_nft(&mut self)-> Result<(), FactoryError> {

            let caller = self.env().caller();

            let is_allowed_mint = self.is_allowed_mint(caller);

            if is_allowed_mint == true {
                return Err(FactoryError::FailedClaim);
            }

            let (nft_type, round) = self.nft_to_claim.get(caller).unwrap_or((0, 0));

            // Claiming must not happen in the same block with minting

            let latest_round = self.get_latest_round();

            if round >= latest_round {
                return Err(FactoryError::FailedClaim);
            }

            if nft_type == 0 {
                let mint = self.mint_chicken(caller);
                if mint.is_err() {
                    return Err(FactoryError::FailedMint);
                }
                // Record last mint for account as chicken
                self.last_mint.insert(caller, &Some((0, mint.unwrap())));
            }
            else {
                let mint = self.mint_fox(caller);
                if mint.is_err() {
                    return Err(FactoryError::FailedMint);
                }
                // Record last mint for account as fox
                self.last_mint.insert(caller, &Some((1, mint.unwrap())));
            }

            Ok(())

        }

        #[ink(message, payable)]
        pub fn mint_nft(&mut self)-> Result<(), FactoryError> {

            let azero_sent = self.env().transferred_value();

            if azero_sent != self.get_azero_for_random_mints() && azero_sent != self.get_azero_for_direct_fox_mints() {
                return Err(FactoryError::InvalidMintPayment);
            }

            let caller = self.env().caller();
            
            let latest_round = self.get_latest_round();

            // Random mint pays AZERO_FOR_RANDOM

            if azero_sent == self.get_azero_for_random_mints() {

                let is_allowed_mint = self.is_allowed_mint(caller);

                if is_allowed_mint == false {
                    return Err(FactoryError::FailedMint);
                }

                let random_number = self.random_int_from_range(1, 10000);
                // Generates a random number and places chances for 80% against 20%
                if random_number >= 1 && random_number < 8000 {
                    // 1 to 8000 range targets chicken
                    self.nft_to_claim.insert(caller, &(0, latest_round));
                    self.allowed_mint.insert(caller, &false);
                    self.update_seed(1);
                }
                else {
                    // 8000 to 10000 range targets fox
                    self.nft_to_claim.insert(caller, &(1, latest_round));
                    self.allowed_mint.insert(caller, &false);
                    self.update_seed(2);
                }

                self.azero_traded += azero_sent;

            }
            else if azero_sent == self.get_azero_for_direct_fox_mints() {
                // Get a defined mint for a Fox (Can only be used twice)
                let direct_fox_mints = self.get_direct_fox_mints(self.env().caller());

                if direct_fox_mints == 2 {
                    return Err(FactoryError::ExceededDirectFoxMintAllowance);
                }

                let mint = self.mint_fox(caller);
                if mint.is_err() {
                    return Err(FactoryError::FailedMint);
                }

                // Record last mint for account as fox
                self.last_mint.insert(caller, &Some((1, mint.unwrap())));
                
                // Increment direct fox mints
                self.direct_fox_mints.insert(caller, &(direct_fox_mints + 1));

                self.azero_traded += azero_sent;

                self.update_seed(3);
                
            }

            // Transfer AZERO to fees account

            let mint_to_admin = (7 * azero_sent) / 100;
            let mint_to_pool = azero_sent - mint_to_admin;

            if self.env().transfer(self.fees_account.unwrap(), mint_to_admin).is_err() {
                return Err(FactoryError::FailedAZEROTransfer);
            }

            if self.env().transfer(self.rewards_pool.unwrap(), mint_to_pool).is_err() {
                return Err(FactoryError::FailedAZEROTransfer);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn pick_random_fox_holder_with_rarity(&self) -> (AccountId, u128) {

            // Generate random number
            
            let random_number:u128 = self.get_fox_for_reward().try_into().unwrap();

            let nft_id;

            let nft: contract_ref!(PSP34) = self.foxes_nft_address.unwrap().into();

            if self.nfts.len() == 0 {
                // No fox has been minted yet
                return (AccountId::from([0u8; 32]), 0);
            }
            
            if let Some(rarities) = self.rarities.get(random_number) {

                // If there is a rarity in mapping with Ids enumerated in it,
                // get a random Fox NFT Id within the rarity vector.

                // Length of items (foxes Ids) that bear the generated rarity value

                let length:u64 = (rarities.len() - 1).try_into().unwrap();

                let random_number:u64 = self.random_int_from_range(0, length);

                let index = usize::try_from(random_number).unwrap();

                // Get the NFT Id from the array representing Ids with the
                // generated rarity value.

                nft_id = rarities[index];

            }
            else {

                // If there are no foxes with the rarity value generated,
                // use a percentile of the rarity / 50 * total number of NFTs minted.
                // Returns an index within that 

                let length:u64 = (self.nfts.len() - 1).try_into().unwrap();
                let _random:u64 = random_number.try_into().unwrap();
                let percentile = (_random * length) / 50;

                let index = usize::try_from(percentile).unwrap();

                // Get the NFT Id at that index
                nft_id = self.nfts[index];
            }

            // Return the owner address of the indexed NFT Id
            (nft.owner_of(Id::U128(nft_id)).unwrap(), nft_id)

        }

        // Gets the last minted NFT from the NFT collection
        #[ink(message)]
        pub fn get_last_mint_by_account(&self, account: AccountId) -> Option<(u8, u128)> {
            self.last_mint.get(account).unwrap_or(None)
        }

        // Gets the number of direct fox mints done by user
        #[ink(message)]
        pub fn get_direct_fox_mints(&self, account: AccountId) -> u8 {
            self.direct_fox_mints.get(account).unwrap_or(0)
        }

        // Gets the total number of NFTs minted: Chickens and Foxes. Returns (chickens count, foxes count)
        pub fn get_minted_nft_count(&self) -> (u128, u128) {
            (self.chickens_minted, self.nfts.len().try_into().unwrap())
        }

        // Generate a random rarity value more inclined to higher rarity values
        #[inline]
        pub fn get_fox_for_reward(&self) -> u64 {
            self._random()
        }

        #[inline]
        pub fn add_rarity(&mut self, rarity:u128, id:u128) {
            // Assign NFT Ids of similar rarity to a like vectors
            if let Some(rarities) = &mut self.rarities.get(rarity) {
                // Push if exists
                rarities.push(id);
                self.rarities.insert(rarity, rarities);
            } else {
                // Create vec if not exists
                self.rarities.insert(rarity, &vec![id]);
            }
            // Assign rarity to NFT Id key
            self.nfts_rarity.insert(id, &rarity);
        }

        // Generate a random rarity value more inclined to lower rarity values
        #[inline]
        pub fn generate_random_rarity(&self) -> u64 {
            51 - self._random()
        }

        // Get latest DIA round number
        #[inline]
        pub fn get_latest_round(&self) -> u64 {
            let round_number = build_call::<DefaultEnvironment>()
            .call(self.oracle.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "RandomOracleGetter::get_latest_round"
            ))))
            .returns::<u64>()
            .try_invoke().unwrap().unwrap();
            round_number
        }

        // Get the rarity of a fox at a given NFT Id
        #[ink(message)]
        pub fn get_fox_rarity(&self, index:u128) -> u128 {
            self.nfts_rarity.get(index).unwrap_or(0)
        }

        #[inline]
        pub fn random_int_from_range(&self, from: u64, to: u64) -> u64 {
            if to == 0{
                return from;
            }

            let round_number = self.get_latest_round();

            let random_value = build_call::<DefaultEnvironment>()
            .call(self.oracle.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "RandomOracleGetter::get_random_value_for_round"
            ))).push_arg(round_number))
            .returns::<Option<Vec<u8>>>()
            .try_invoke().unwrap().unwrap().unwrap();

            let prefix = u64::from_ne_bytes(random_value[0..8].try_into().unwrap());

            let mut source = random::default(self.seed + prefix + round_number);
            let rand_int:u64 = source.read::<u64>() % to + from;
            rand_int
        }

        // Biased random number generator: 1 to 50
        #[inline]
        fn _random(&self) -> u64 {

            // Arithmetic Progression (AP) formula (52 nth term) = a + (n - 1) * n = 2653

            let random_number = self.random_int_from_range(1, 2653);

            // Using AP, classify values in cluster ranges (larger ranges for smaller values)
            // between 1 and 50.

            let mut _last_index = 0;
            let mut index = 0;
            
            // Loop to get the number associated with the ranged random value
            for i in 1..= 52 {
                if _last_index != 0 {
                    index += 1;
                    let _current_index = 1 + (i - 1) * (i + (10 * 1));
                    // Check if index falls within the range in AP
                    if random_number > _last_index && random_number <= _current_index {
                        break;
                    }
                }
                _last_index = 1 + (i - 1) * (i + (10 * 1))
            }

            // Ranging makes results most probable to fall into smaller ranges

            let return_value = 52 - index;

            // Invert result

            return_value

        }

        // Mints a fox to 'account'
        #[inline]
        pub fn mint_fox(&mut self, account: AccountId) -> Result<u128, PSP34Error> {

            let index:u128 = (self.nfts.len() + 1).try_into().unwrap();

            let rarity:u128 = self.generate_random_rarity().try_into().unwrap();

            // Generate a rarity value: Biased to most likely return smaller rarity values

            self.nfts.push(index);

            // Push NFT to vec of NFTs minted

            self.add_rarity(rarity, index);

            // Associated rarity to NFT Id

            let mut nft: contract_ref!(PSP34Mintable) = self.foxes_nft_address.unwrap().into(); // Ref {} of foxes NFT contract

            let key = String::from("rarity").as_bytes().to_vec(); // String to Vec<u8>
            let value = rarity.to_be_bytes().to_vec(); // u128 to Vec<u8>

            let attributes = vec![(key, value)]; // Construct attributes

            nft.mint_with_attributes(account, attributes)?;

            // Mint the fox with rarity attribute

            Ok(index)

        }

        // Mints a chicken to 'account'
        #[inline]
        pub fn mint_chicken(&mut self, account: AccountId) -> Result<u128, PSP34Error> {

            // Ref {} of chickens NFT
            
            let mut nft: contract_ref!(PSP34Mintable) = self.chickens_nft_address.unwrap().into();

            // Mint chicken
            nft.mint(account)?;

            let index:u128 = self.chickens_minted + 1;

            self.chickens_minted = index;

            Ok(index)

        }

    }
}