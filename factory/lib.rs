#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod factory {

    use ink::prelude::{
        vec::Vec,
        string::String
    };

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
        OnlyOwnerAllowed
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
        last_mint: Mapping<AccountId, u8>
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
                last_mint: Mapping::default()
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

        #[ink(message)]
        pub fn generate_random_nft(&mut self)-> Result<(), FactoryError> {
            let mut source = random::default(self.env().block_timestamp());
            let caller = self.env().caller();
            let random_number = source.read::<u64>() % 10000 + 1;
            // Generates a random number and places chances for 90% against 10%
            if random_number >= 1 && random_number < 9000 {
                // 1 to 9000 range targets chicken
                if self.mint_chicken(caller).is_err() {
                    return Err(FactoryError::FailedMint);
                }
                self.last_mint.insert(caller, &0);
            }
            else {
                // 9000 to 10000 range targets fox
                if self.mint_fox(caller).is_err() {
                    return Err(FactoryError::FailedMint);
                }
                self.last_mint.insert(caller, &1);
            }
            Ok(())
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

                let mut source = random::default(self.env().block_timestamp());

                // Length of items (foxes Ids) that bear the generated rarity value

                let length:u64 = (rarities.len() - 1).try_into().unwrap();
                let random_number:u64 = source.read::<u64>() % length;
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

        // Biased random number generator: 1 to 50
        #[inline]
        fn _random(&self) -> u64 {

            let mut source = random::default(self.env().block_timestamp());

            // Arithmetic Progression (AP) formula (52 nth term) = a + (n - 1) * n = 2653

            let random_number = source.read::<u64>() % 2653 + 1;

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
        pub fn mint_fox(&mut self, account: AccountId) -> Result<(), PSP34Error> {

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

            Ok(())

        }

        // Mints a chicken to 'account'
        #[inline]
        pub fn mint_chicken(&mut self, account: AccountId) -> Result<(), PSP34Error> {

            // Ref {} of chickens NFT
            
            let mut nft: contract_ref!(PSP34Mintable) = self.chickens_nft_address.unwrap().into();

            // Mint chicken
            nft.mint(account)?;

            Ok(())

        }

    }
}