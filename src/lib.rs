use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{WrappedBalance, WrappedDuration, WrappedTimestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json, Value};
use near_sdk::{
    env, ext_contract, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, Duration,
    Gas, PanicOnDefault, Promise, PromiseResult, Timestamp,
};

pub const DEFAULT_GAS_TO_PAY: Gas = 20_000_000_000_000;

pub use crate::admin::*;
pub use crate::categories::*;
pub use crate::ext::*;
pub use crate::json_types::*;
pub use crate::types::*;
pub use crate::views::*;

pub use crate::requester_action::*;
pub use crate::user::*;
pub use crate::worker_action::*;

pub use crate::utils::*;

mod admin;
mod categories;
mod ext;
mod json_types;
mod types;
mod views;

mod requester_action;
mod user;
mod worker_action;

mod utils;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Dwork {
    task_recores: UnorderedMap<TaskId, Task>,
    users: LookupMap<AccountId, User>,
    categories: UnorderedMap<CategoryId, Category>,
    admins: LookupSet<AccountId>,
    app_config: AppConfig,
}

//NOTE: We do not keep the submitted bond as a locked balance.
#[near_bindgen]
impl Dwork {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized",);

        Self {
            task_recores: UnorderedMap::new(StorageKey::TaskRecores),
            users: LookupMap::new(StorageKey::Users),
            categories: UnorderedMap::new(StorageKey::Categories),
            admins: LookupSet::new(StorageKey::Admins),
            app_config: AppConfig::default(),
        }
    }

    //TODO: Define later
    // pub fn leave(&mut self) {
    //     let account_id = env::predecessor_account_id();
    //     let user = self.users
    //         .get(&account_id)
    //         .expect("You are not a member of dWork");
    //
    //     Promise::new(env::predecessor_account_id()).transfer(self.app_config.register_bond);
    //     self.users.remove(&account_id);
    // }
}
