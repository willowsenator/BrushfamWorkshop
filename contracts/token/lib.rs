#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp22 {
    
    // imports from openbrush
	use openbrush::contracts::psp22::*;
	use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Token {
    	#[storage_field]
		psp22: psp22::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for Token {}
     
    impl Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			_instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
			_instance
        }
    }

#[cfg(test)]
mod tests{
    use super::*;
    use openbrush::test_utils::*;

    #[ink::test]
    fn contructor_works(){
        let total_supply = 5_000_000;
        let accounts = accounts();

        let contract = Token::new(total_supply);
        assert_eq!(contract.total_supply(), total_supply);
        assert_eq!(contract.balance_of(accounts.alice), total_supply);
    }
    
    #[ink::test]
    fn transfer_works(){
        let total_supply = 3_000_000;
        let transferred_amount = 1000;
        let accounts = accounts();

        let mut contract = Token::new(total_supply);

        let tx = contract.transfer(accounts.bob, transferred_amount, vec![]);
        assert!(tx.is_ok());

        assert!(contract.balance_of(accounts.alice) == total_supply - transferred_amount);
        assert_eq!(contract.balance_of(accounts.bob), transferred_amount);
    }
}
}