use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum RunningState {
    Running, Paused
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AppConfig {
    pub running_state: RunningState,
    pub register_bond: Balance,
    pub submit_bond: Balance,
    pub minimum_reward_per_task: Balance,
    pub maximum_reward_per_task: Balance,
    pub maximum_description_length: u16,
    pub maximum_cover_letter_length: u16,
    pub maximum_proposals_at_one_time: u16,
    pub maximum_requests_active_per_user: u16,
    pub maximum_title_length: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
       Self {
           running_state: RunningState::Running,
           register_bond: 500_000_000_000_000_000_000_000,
           submit_bond: 10_000_000_000_000_000_000_000,
           minimum_reward_per_task: 10_000_000_000_000_000_000_000,
           maximum_reward_per_task: 100_000_000_000_000_000_000_000_000,
           maximum_description_length: 10000,
           maximum_cover_letter_length: 10000,
           maximum_proposals_at_one_time: 200,
           maximum_requests_active_per_user: 10,
           maximum_title_length: 100
       } 
    }
}

#[near_bindgen]
impl Dwork {
    pub fn add_admin(&mut self, account_id: AccountId) -> bool {
        let caller_id = env::predecessor_account_id();
        let contract_id = env::current_account_id();
        assert!(
            caller_id == "neilharan.testnet" 
            || caller_id == contract_id,
            "You not have permission to give admin rights"); 
        assert!(!self.is_admin(account_id.clone()), "This account already have admin rights");
        self.admins.insert(&account_id)
    }

    pub fn remove_admin(&mut self, account_id: AccountId) -> bool {
        let caller_id = env::predecessor_account_id();
        let contract_id = env::current_account_id();
        assert!(
            caller_id == "neilharan.testnet" 
            || caller_id == contract_id,
            "You not have permission to remove admin rights"); 
        assert!(self.is_admin(account_id.clone()), "This account not have admin rights");
        self.admins.remove(&account_id)
    }

    pub fn is_admin(&self, account_id: AccountId) -> bool {
        self.admins.contains(&account_id)
    }

    // //NOTE: Migrate function
    // #[private]
    // #[init(ignore_state)]
    // pub fn migrate(&self) -> Self {
    //     assert!(self.is_admin(env::predecessor_account_id()), "You don't have permission to migrate this contract!");
    //     let old_state: OldContract = env::state_read().expect("failed");
    //     Self {
    //         storage_accounts: old_state.storage_accounts,
    //         accounts: old_state.accounts,
    //         posts: old_state.posts,
    //         user_posts: old_state.user_posts,
    //         deleted_posts: old_state.deleted_posts,
    //         messages: old_state.messages,
    //         likes: old_state.likes,
    //         comments: old_state.comments,
    //         topics: old_state.topics,
    //         topics_posts: old_state.topics_posts,
    //         communities: old_state.communities,
    //         communities_posts: old_state.communities_posts,
    //         members_in_communites: old_state.members_in_communites,
    //         storage_account_in_bytes: old_state.storage_account_in_bytes,
    //         admins: LookupSet::new(StorageKey::Admins),
    //     }
    // }
}
