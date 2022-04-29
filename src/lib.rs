use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise, PromiseOrValue, PanicOnDefault};
use near_sdk::collections::UnorderedSet;
use near_sdk::Duration;

use std::time::{SystemTime, UNIX_EPOCH};

pub type AccountId = String;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Lottery {
    /// Lottery name
    pub lottery_name: String,
    /// End date of the lottery
    pub end_date: Duration,
    /// Participants
    pub participants: UnorderedSet<AccountId>,
}


fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::predecessor_account_id(),
        "Can only be called by owner"
    );
}

fn assert_end(){
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
}

#[near_bindgen]
impl Lottery {
    #[init]
    pub fn new(lottery_name: String, end_date: Duration) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            lottery_name,
            end_date,
            participants: UnorderedSet::new(b"a".to_vec()),
        }
    }

    pub fn enter(&mut self, account_id:AccountId){
        assert!(
            !self.participants.contains(&account_id),
            "You are already participating in the lottery."
        );

        self.participants.insert(&account_id);
    }

    pub fn get_participans(&self) -> std::vec::Vec<AccountId>{
        let par = self.participants.to_vec();
        return par;
    }

    pub fn get_end_date(&mut self) -> Duration {
        return self.end_date;
    }
    
    pub fn get_winner(&mut self) -> AccountId {
        
    }
}