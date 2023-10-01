#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod organization {
    use scale::{Decode, Encode};
    use ink::storage::Mapping;

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Contributor {
        // the contributor identifier
        address: AccountId,
        // the contributor score is the reputation between 0-10
        // where 0 is the worst and 10 is the best.
        score: u64,
    }

    impl Contributor {}

    #[ink(storage)]
    pub struct OrgStorage {
        admin: AccountId,
        contributors: Mapping<AccountId, Contributor>
    }    

    impl OrgStorage {        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                contributors: Mapping::default(),
            }
        }

        // add contributors create a new contributor in the storage
        // according to the account id caller
        #[ink(message, payable)]
        pub fn addcontributor(&mut self, score: u64) {
            let caller = self.env().caller();
            let contributor = Contributor{
                address: caller,
                score: score,
            };
            self.contributors.insert(caller, &contributor);
        }

        #[ink(message)]
        pub fn getcontributor(&self) -> Option<Contributor> {
            let caller = self.env().caller();
            self.contributors.get(caller)
        }
    }
}
