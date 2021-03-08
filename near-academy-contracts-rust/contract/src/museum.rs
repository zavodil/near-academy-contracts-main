use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

pub type Timestamp = u64;

const MIN_ACCOUNT_BALANCE: u128 = 3_000_000_000_000_000_000_000_000;
//const XCC_GAS: u128 = 20_000_000_000_000;

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


    /**
    * Add your memeCategory
    */
    #[payable]
    pub fn add_meme(&mut self, meme: AccountId, title: String, data: String, category: Category) {
        assert!(self.is_contributor() || self.is_owner(),
                "This method can only be called by a museum contributor or owner");

        assert!(
            env::attached_deposit() > MIN_ACCOUNT_BALANCE,
            "Minimum account balance must be attached to initialize a meme (3 NEAR)");
    }
}







/*
export function add_meme(
meme: AccountId,
title: string,
data: string,
category: Category
): void {
assert_contract_is_initialized()
assert_signed_by_contributor_or_owner()

// storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
assert(
u128.ge(context.attachedDeposit, MIN_ACCOUNT_BALANCE),
"Minimum account balance must be attached to initialize a meme (3 NEAR)"
);

const accountId = full_account_for(meme)

assert(env.isValidAccountID(accountId), "Meme name must be valid NEAR account name")
assert(!Museum.has_meme(accountId), "Meme name already exists")

logging.log("attempting to create meme")

let promise = ContractPromiseBatch.create(accountId)
.create_account()
.deploy_contract(Uint8Array.wrap(changetype<ArrayBuffer>(CODE)))
.add_full_access_key(base58.decode(context.senderPublicKey))

promise.function_call(
"init",
new MemeInitArgs(title, data, category),
context.attachedDeposit,
XCC_GAS
)

promise.then(context.contractName).function_call(
"on_meme_created",
new MemeNameAsArg(meme),
u128.Zero,
XCC_GAS
)
}
*/