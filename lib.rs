#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use psp22::{PSP22Data, PSP22Event};
pub use psp22::PSP22;
pub use psp22::PSP22Error;

pub use self::token::Token;

/// PSP22 token implementation.
///
/// This struct represents a PSP22 compliant fungible token.
#[ink::contract]
pub mod token {
    use psp22::PSP22Data;

    #[ink(storage)]
    pub struct Token {
        pub data: PSP22Data,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(
            supply: u128,
        ) -> Self {
            Self {
                data: PSP22Data::new(supply, Self::env().caller()),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }
    }
}
