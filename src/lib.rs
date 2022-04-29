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
        return self.participants.to_vec();
    }

    pub fn get_num_participans(&self) -> u64{
        return self.participants.len();
    }

    pub fn get_end_date(&self) -> Duration {
        return self.end_date;
    }

    pub fn get_lottery_name(&self) -> String {
        return self.lottery_name.clone();
    }
    
    // pub fn get_winner(&mut self) -> AccountId {
    //     assert!(assert_end(self.end_date), "Not finished");
    // }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests{
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use std::panic;
    use super::*;

    fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
        f: F,
    ) -> std::thread::Result<R> {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        result
    }

    fn get_context() -> VMContext {
        VMContext {
            current_account_id: "alice".to_string(),
            signer_account_id: "bob".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob".to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(15),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_new() {
        let context = get_context();
        testing_env!(context);
        let lottery_name = "Test".to_string();
        let end_date = 1651249778;
        let contract = Lottery::new(lottery_name.clone(), end_date);
        assert_eq!(contract.get_lottery_name(), lottery_name);
        assert_eq!(contract.get_end_date(), end_date);
        assert_eq!(contract.get_num_participans(), 0);
    }
}