// Raen social
//
//

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::{UnorderedMap, Vector};

near_sdk::setup_alloc!();


pub struct Message {
    id: u128,
    text: String,
    timestamp: u64
}

impl Message {
    pub fn default(text: String) {
        text.to_string();
        timestamp = env::block_timestamp;
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GetNear {
    posts: UnorderedMap<AccountId, Vector<Message>>,
    likes: u64,
}

#[near_bindgen]
impl GetNear {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Not a valid account!");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            posts: UnorderedMap::new(b"token-belongs-to".to_vec()),
            tokens: 1,
        }
    }

    pub fn post_message(&mut self, message: Message) {
        env::log(b"sending message");
        self.posts.insert(env::signer_account_id, self.posts.get(env::signer_account_id).push(&message));
    }

    pub fn read_messages_by_account(&self, account_id: AccountId) -> Vector<Message> {
        assert!(env::is_valid_account_id(account_id.as_bytes()), "Not a valid account!");
        env::log(b"reading message of account: {account_id}");
        const messages: Vector<Message> = self.posts.get(&account_id);
        messages
    }

    pub fn delete_message(&self, message: Message) -> bool {
        for msg in self.posts.get(&message) {
            match &message {
                Some => self.posts.pop(msg),
                None => continue
            };
        }
        true
    }

    pub fn get_message(message: Message) -> Message {
        for msg in self.posts.get(&message) {

        }
    }
}


fn is_owner(account_id: AccountId) -> bool {
    assert_eq!(account_id, env)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{ testing_env, VMContext };

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn post_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = GetNear::new();
        let mut text = Message::new("First message.");
        contract.post_message(context.predecessor_account_id, text);
        assert_eq!(contract.posts.read())
    }
}