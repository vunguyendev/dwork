use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{WrappedBalance, WrappedDuration, WrappedTimestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json, Value};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, Duration,
    Gas, PanicOnDefault, Promise, Timestamp, StorageUsage,
};

pub const DEFAULT_GAS_TO_PAY: Gas = 20_000_000_000_000;

pub use crate::admin::*;
pub use crate::categories::*;
pub use crate::ext::*;
pub use crate::json_types::*;
pub use crate::types::*;
pub use crate::views::*;

pub use crate::requester_action::*;
pub use crate::account::*;
pub use crate::worker_action::*;
pub use crate::task::*;
pub use crate::report::*;

pub use crate::storage::*;
pub use crate::utils::*;

mod admin;
mod categories;
mod ext;
mod json_types;
mod types;
mod views;

mod requester_action;
mod account;
mod worker_action;
mod task;
mod report;

mod utils;
mod storage;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Dwork {
    pub storage_accounts: LookupMap<AccountId, StorageAccount>,
    pub accounts: LookupMap<AccountId, Account>,
    
    pub admins: LookupSet<AccountId>,
    pub storage_account_in_bytes: StorageUsage,
    pub app_config: AppConfig,

    pub task_recores: UnorderedMap<TaskId, Task>,
    pub proposals: LookupMap<ProposalId, Proposal>,
    pub reports: UnorderedMap<ReportId, Report>,
    
    pub categories: UnorderedMap<CategoryId, Category>,
}

//NOTE: We do not keep the submitted bond as a locked balance.
#[near_bindgen]
impl Dwork {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized",);

        Self {
            storage_accounts: LookupMap::new(StorageKey::StorageAccount),
            accounts: LookupMap::new(StorageKey::Users),
            
            admins: LookupSet::new(StorageKey::Admins),
            storage_account_in_bytes: 0,
            app_config: AppConfig::default(),
            
            task_recores: UnorderedMap::new(StorageKey::TaskRecores),
            proposals: LookupMap::new(StorageKey::Proposals),
            reports: UnorderedMap::new(StorageKey::Reports),
            
            categories: UnorderedMap::new(StorageKey::Categories),
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
