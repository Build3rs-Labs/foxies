#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod swap {

    use ink::prelude::{
        vec::Vec
    };

    use ink::prelude::vec;

    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment,
            CallFlags
        }
    };

    use ink::contract_ref;

    use psp22::PSP22;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum SwapError {
        /// Thrown when there isn't sufficient liquidity
        InsufficientLiquidity,
        /// Thrown when the caller must be owner
        OnlyOwnerAllowed,
        /// Thrown when there is insufficient allowance
        InsufficientAllowance,
        /// Thrown when there is insufficient balance
        InsufficientBalance,
        /// Thrown when 0 amount is supplied
        ZeroAmount,
        /// Thrown when swap fails
        SwapFailed,
        /// Thrown when a transfer fails
        TransferFailed
    }

    #[ink(storage)]
    pub struct Swap {
        pool_address: Option<AccountId>,
        owner: Option<AccountId>,
        token_address: Option<AccountId>
    }

    impl Swap {
        #[ink(constructor)]
        pub fn new(pool_address: AccountId, token_address: AccountId) -> Self {
            let owner = Self::env().caller();
            Self {
                owner: Some(owner),
                pool_address: Some(pool_address),
                token_address: Some(token_address)
            }
        }
        
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            Self::env().account_id()
        }

        #[ink(message)]
        pub fn set_pool_address(&mut self, pool_address: AccountId) -> Result<(), SwapError> {
            if self.env().caller() != self.owner.unwrap() {
                return Err(SwapError::OnlyOwnerAllowed);
            }
            self.pool_address = Some(pool_address);
            Ok(())
        }

        #[ink(message)]
        pub fn get_eggs_for_azero(&self, amount_azero: Balance) -> Balance {

            let amount_in = amount_azero - ((15 * amount_azero) / 1000); //1.5% fees total

            // Get amount of eggs to give per given amount of AZERO

            let amount_eggs = build_call::<DefaultEnvironment>()
            .call(self.pool_address.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "get_psp22_amount_out_without_deduction"
            ))).push_arg(amount_in))
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .returns::<Balance>()
            .try_invoke().unwrap().unwrap();

            amount_eggs

        }

        #[ink(message)]
        pub fn get_azero_for_eggs(&self, amount_eggs: Balance) -> Balance {

            let amount_in = amount_eggs - ((15 * amount_eggs) / 1000); //1.5% fees total

            // Get amount of eggs to give per given amount of AZERO

            let amount_azero = build_call::<DefaultEnvironment>()
            .call(self.pool_address.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "get_a0_amount_out_without_deduction"
            ))).push_arg(amount_in))
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .returns::<Balance>()
            .try_invoke().unwrap().unwrap();

            amount_azero
        }

        #[inline]
        pub fn get_pool_balances(&self) -> Vec<Balance> {

            // Gets the balance of AZERO and EGGS in the pool contract

            let token_balances = build_call::<DefaultEnvironment>()
            .call(self.pool_address.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "get_token_balances"
            ))))
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .returns::<Vec<Balance>>()
            .try_invoke().unwrap().unwrap();

            //returns [$AZERO in pool, $EGGS in pool]

            token_balances
            
        }

        #[ink(message)]
        pub fn swap_eggs_for_azero(&mut self, amount: Balance) -> Result<(), SwapError> {

            let amount_eggs = amount;

            let mut psp22: contract_ref!(PSP22) = self.token_address.unwrap().into();

            let user_balance = psp22.balance_of(self.env().caller());
            let user_allowance = psp22.allowance(self.env().caller(), Self::env().account_id());

            // Checks for user allowance and balance
            
            if user_allowance < user_balance {
                return Err(SwapError::InsufficientAllowance);
            }
            if amount_eggs > user_balance {
                return Err(SwapError::InsufficientBalance);
            }
            if amount_eggs == 0 {
                return Err(SwapError::ZeroAmount);
            }

            let _ = psp22.transfer_from(self.env().caller(), self.env().account_id(), amount_eggs, vec![]);

            let foxies_fees = (7 * amount_eggs) / 1000; // 0.7% fee

            let amount_to_swap = amount_eggs - foxies_fees;

            // Get expected AZERO to get for PSP22 swap

            let azero_to_give_out = self.get_azero_for_eggs(amount_to_swap);

            // Get $AZERO and $EGGS in pool
            let balances = self.get_pool_balances();

            let azero_in_pool = balances[usize::try_from(0).unwrap()]; // $AZERO in pool

            if azero_to_give_out > azero_in_pool {
                return Err(SwapError::InsufficientLiquidity); // Amount to be given out should not exceed liquidity balance
            }

            let _ = psp22.transfer(self.owner.unwrap(), foxies_fees, vec![]);

            let _ = psp22.approve(self.pool_address.unwrap(), amount_to_swap * 10);

            if build_call::<DefaultEnvironment>()
            .call(self.pool_address.unwrap())
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "swap_psp22"
            )))
            
            .push_arg(amount_to_swap - 1000)
            .push_arg(azero_to_give_out)
            .push_arg(10000 * 10u128.pow(12)))
            .call_flags(CallFlags::default().set_allow_reentry(true))

            .returns::<()>()
            .try_invoke().is_err() {
                return Err(SwapError::SwapFailed); // Swap failed
            }

            let azero_balance = self.env().balance();

            let _ = self.env().transfer(self.env().caller(), azero_balance);

            Ok(())
        }

        #[ink(message, payable)]
        pub fn swap_azero_for_eggs(&mut self) -> Result<(), SwapError> {
            let amount_azero = self.env().transferred_value();
            if amount_azero == 0 {
                return Err(SwapError::ZeroAmount);
            }

            let foxies_fees = (7 * amount_azero) / 1000;

            let amount_to_swap = amount_azero - foxies_fees;

            let eggs_to_give_out = self.get_eggs_for_azero(amount_to_swap);

            // Get $AZERO and $EGGS in pool
            let balances = self.get_pool_balances();

            let eggs_in_pool = balances[usize::try_from(1).unwrap()]; // $EGGS in pool

            if eggs_to_give_out > eggs_in_pool {
                return Err(SwapError::InsufficientLiquidity); // Amount to be given out should not exceed liquidity balance
            }

            let _ = self.env().transfer(self.owner.unwrap(), foxies_fees);

            if build_call::<DefaultEnvironment>()
            .call(self.pool_address.unwrap())
            .transferred_value(amount_to_swap)
            .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                "swap_a0"
            )))
            
            .push_arg(eggs_to_give_out)
            .push_arg(10000 * 10u128.pow(12)))
            .call_flags(CallFlags::default().set_allow_reentry(true))

            .returns::<()>()
            .try_invoke().is_err() {
                return Err(SwapError::SwapFailed); // Swap failed
            }

            let mut psp22: contract_ref!(PSP22) = self.token_address.unwrap().into();

            let eggs_balance = psp22.balance_of(self.env().account_id());

            let _ = psp22.transfer(self.env().caller(), eggs_balance, vec![]);

            Ok(())
        }

    }
}