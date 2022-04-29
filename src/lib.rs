use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedSet;

pub type AccountId = String;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Faucet {
    /// Lottery name
    pub lottery_name: AccountId,
    /// End date of the lottery
    pub end_date: u64,
    /// Participants
    pub participants: UnorderedSet<AccountId>,
}