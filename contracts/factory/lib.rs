#![cfg_attr(not(feature = "std"), no_std, no_main)]

const AZERO_FOR_RANDOM:u128 = 6 * 10u128.pow(12); // 6 AZERO for random
const AZERO_FOR_DIRECT_FOX_MINT:u128 = 100 * 10u128.pow(12); // 100 AZERO for defined fox mint

#[ink::contract]
mod factory {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

    use crate::AZERO_FOR_RANDOM;
    use crate::AZERO_FOR_DIRECT_FOX_MINT;

    use random::Source;

    use ink::storage::Mapping;

    use ink::prelude::vec;

    use ink::contract_ref;

    use foxes::{
        PSP34Mintable,
        PSP34Error,
        PSP34,
        Id
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum FactoryError {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        /// Returned when mint has failed (maybe total supply netted)
        FailedMint,
        /// Returned when caller is not owner for owner-only methods
        OnlyOwnerAllowed,
        /// Returned when amount sent is not valid payment
        InvalidMintPayment,
        /// Returned when a user has exceeded their max direct fox mints
        ExceededDirectFoxMintAllowance
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
        // AZERO for random mints: Mutable
        azero_for_random_mints: u128
    }

    impl Factory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
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
                azero_for_random_mints: AZERO_FOR_RANDOM
            }
        }

        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            Self::env().account_id()
        }

        // Set chickens NFT address: Only manager can call this method
        #[ink(message)]
        pub fn set_chickens_nft_address(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.chickens_nft_address = Some(address);
            Ok(())
        }

        // Set foxes NFT address: Only manager can call this method
        #[ink(message)]
        pub fn set_foxes_nft_address(&mut self, address: AccountId) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.foxes_nft_address = Some(address);
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

        // Set the amount of AZERO for random mints: Only manager can call this method
        #[ink(message)]
        pub fn set_azero_for_random_mints(&mut self, amount: u128) -> Result<(), FactoryError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(FactoryError::OnlyOwnerAllowed);
            }
            self.azero_for_random_mints = amount;
            Ok(())
        }

        #[ink(message, payable)]
        pub fn generate_random_nft(&mut self)-> Result<(), FactoryError> {

            let azero_sent = self.env().transferred_value();

            if azero_sent != self.azero_for_random_mints && azero_sent != self.azero_for_direct_fox_mints {
                return Err(FactoryError::InvalidMintPayment);
            }

            let caller = self.env().caller();

            // Random mint pays AZERO_FOR_RANDOM

            if azero_sent == self.azero_for_random_mints {
                let random_number = self.random_int_from_range(1, 10000);
                // Generates a random number and places chances for 80% against 20%
                if random_number >= 1 && random_number < 8000 {
                    // 1 to 8000 range targets chicken
                    let mint = self.mint_chicken(caller);
                    if mint.is_err() {
                        return Err(FactoryError::FailedMint);
                    }
                    // Record last mint for account as chicken
                    self.last_mint.insert(caller, &Some((0, mint.unwrap())));
                }
                else {
                    // 8000 to 10000 range targets fox
                    let mint = self.mint_fox(caller);
                    if mint.is_err() {
                        return Err(FactoryError::FailedMint);
                    }
                    // Record last mint for account as fox
                    self.last_mint.insert(caller, &Some((1, mint.unwrap())));
                }
                Ok(())
            }
            else {
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

                Ok(())
                
            }
        }

        #[ink(message)]
        pub fn pick_random_fox_holder_with_rarity(&self) -> AccountId {

            // Generate random number
            
            let random_number:u128 = self.get_fox_for_reward().try_into().unwrap();

            let nft_id;

            let nft: contract_ref!(PSP34) = self.foxes_nft_address.unwrap().into();

            if self.nfts.len() == 0 {
                // No fox has been minted yet
                return AccountId::from([0u8; 32]);
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
            nft.owner_of(Id::U128(nft_id)).unwrap()

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

        // Get the rarity of a fox at a given NFT Id
        #[ink(message)]
        pub fn get_fox_rarity(&self, index:u128) -> u128 {
            self.nfts_rarity.get(index).unwrap_or(0)
        }

        #[inline]
        fn random_int_from_range(&self, from: u64, to: u64) -> u64 {
            let mut source = random::default(self.env().block_timestamp());
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