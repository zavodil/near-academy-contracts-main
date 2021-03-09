use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::{AccountId, Promise, Balance, Gas};

pub type Timestamp = u64;


const MIN_ACCOUNT_BALANCE: u128 = 3_000_000_000_000_000_000_000_000;
const XCC_GAS: Gas = 20_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Museum {
    museum_name: String,
    created_at: Timestamp,

    owners: Vec<AccountId>,
    memes: Vec<AccountId>,
    contributors: Vec<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Category {
    A,
    B,
    C,
    D,
}

pub struct MemeInitArgs {
    title: String,
    data: String,
    category: Category,
}

impl Museum {
    pub fn new(museum_name: String, created_at: Timestamp) -> Self {
        Self {
            museum_name,
            created_at,

            owners: Vec::new(),
            memes: Vec::new(),
            contributors: Vec::new(),
        }
    }

    pub fn get_owner_list(&self) -> Vec<AccountId> {
        self.owners.clone()
    }

    pub fn get_meme_list(&self) -> Vec<AccountId> {
        self.memes.clone()
    }

    pub fn get_meme_count(&self) -> usize {
        self.memes.len()
    }

    pub fn is_contributor(&self, account: AccountId) -> bool {
        self.contributors.contains(&account)
    }

    pub fn add_myself_as_contributor(&mut self) {
        self.contributors.push(env::predecessor_account_id())
    }

    pub fn remove_myself_as_contributor(&mut self) {
        let current_user = env::predecessor_account_id();
        let index = self.contributors.iter().position(|user| *user == current_user).unwrap();
        self.contributors.remove(index);
    }

    pub fn has_owner(&self, account: AccountId) -> bool {
        self.owners.contains(&account)
    }

    pub fn has_meme(&self, account: AccountId) -> bool {
        self.memes.contains(&account)
    }

    pub fn add_meme(&mut self, account: AccountId) {
        self.memes.push(account)
    }
}

impl Default for Museum {
    fn default() -> Self {
        env::panic(b"Museum should be initialized before usage")
    }
}


#[near_bindgen]
impl MemeMuseum {
    pub fn get_museum(&self) -> Museum {
        self.museum.clone()
    }

    pub fn get_owner_list(&self) -> Vec<AccountId> {
        self.museum.get_owner_list()
    }

    pub fn get_meme_list(&self) -> Vec<AccountId> {
        self.museum.get_meme_list()
    }

    pub fn get_meme_count(&self) -> usize {
        self.museum.get_meme_count()
    }


    /**
    * Manage your status as a contributor
    */
    pub fn add_myself_as_contributor(&mut self) {
        self.museum.add_myself_as_contributor()
    }

    pub fn remove_myself_as_contributor(&mut self) {
        self.museum.remove_myself_as_contributor()
    }

    pub fn is_contributor(&self) -> bool {
        self.museum.is_contributor(env::predecessor_account_id())
    }

    pub fn is_owner(&self) -> bool {
        self.museum.has_owner(env::predecessor_account_id())
    }

    pub fn full_account_for(&self, meme: String) -> String {
        format!("{}.{}", meme, &env::current_account_id())
    }


    /**
    * * Add your meme
    */
    #[payable]
    pub fn add_meme(&mut self, meme: AccountId, title: String, data: String, category: Category) -> Promise {
        assert!(self.is_contributor() || self.is_owner(),
                "This method can only be called by a museum contributor or owner");

        let deposit: Balance = env::attached_deposit();
        assert!(
            deposit > MIN_ACCOUNT_BALANCE,
            "Minimum account balance must be attached to initialize a meme (3 NEAR)");

        let account_id = self.full_account_for(meme.clone());


        assert!(env::is_valid_account_id(account_id.as_bytes()),
                "Meme name must be valid NEAR account name");
        assert!(!self.museum.has_meme(account_id.clone()),
                "Meme name already exists");

        env::log(format!("Attempting to create meme {}", account_id.clone()).as_bytes());

        let promise = Promise::new(account_id.clone());

        let current_contract = env::current_account_id();
        let params = serde_json::to_vec(&json!({
                    "title": title,
                    "data": data,
                    "category": category,
                })).unwrap();

        promise
            .create_account()
            .add_full_access_key(env::signer_account_pk().into())
            .function_call(
                "init".into(),
                params,
                deposit,
                XCC_GAS,
            ).then(
            ext_self::on_add_meme(
                meme,
                &current_contract,
                NO_DEPOSIT,
                XCC_GAS,
            ))
    }

    pub fn on_add_meme(&mut self, meme: AccountId) -> bool {
        let init_meme_succeeded = is_promise_success();
        if !init_meme_succeeded {
            env::log(format!("Init failed").as_bytes());
            false
        } else {
            self.museum.add_meme(meme);
            true
        }
    }

    pub fn add_contributor(&mut self, account: AccountId) {
        self.assert_contract_is_initialized();
        self.assert_signed_by_owner();

        self.museum.contributors.push(account.clone());

        env::log(format!("Contributor {} was added", account.clone()).as_bytes());
    }

    pub fn assert_signed_by_owner(&self) {
        assert!(self.is_owner(), "This method can only be called by a museum owner")
    }

    pub fn assert_contract_is_initialized(&self) {
        assert!(self.is_initialized(), "Contract must be initialized first.");
    }

    pub fn is_initialized(&self) -> bool {
        env::state_exists()
    }
}
