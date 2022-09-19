use core::fmt;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum RunningState {
    Running,
    Paused,
}

impl fmt::Display for RunningState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunningState::Running => write!(f, "Running"),
            RunningState::Paused => write!(f, "Paused"),
        }
    }
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
            maximum_title_length: 100,
        }
    }
}

#[near_bindgen]
impl Dwork {
    pub fn is_admin(&self, account_id: AccountId) -> bool {
        self.admins.contains(&account_id)
    }

    //Change config by admin
    pub fn change_config(
        &mut self,
        register_bond: Option<Balance>,
        submit_bond: Option<Balance>,
        min_reward: Option<Balance>,
        max_reward: Option<Balance>,
    ) {
        if let Some(register_bond) = register_bond {
            self.app_config.register_bond = register_bond;
        }

        if let Some(submit_bond) = submit_bond {
            self.app_config.submit_bond = submit_bond;
        }

        if let Some(min_reward) = min_reward {
            self.app_config.minimum_reward_per_task = min_reward;
        }

        if let Some(max_reward) = max_reward {
            self.app_config.maximum_reward_per_task = max_reward;
        }
    }

    pub fn add_admin(&mut self, account_id: AccountId) -> bool {
        let caller_id = env::predecessor_account_id();
        let contract_id = env::current_account_id();
        assert!(
            caller_id == "neilharan.testnet" || caller_id == contract_id,
            "You not have permission to give admin rights"
        );
        assert!(
            !self.is_admin(account_id.clone()),
            "This account already have admin rights"
        );
        self.admins.insert(&account_id)
    }

    pub fn remove_admin(&mut self, account_id: AccountId) -> bool {
        let caller_id = env::predecessor_account_id();
        let contract_id = env::current_account_id();
        assert!(
            caller_id == "neilharan.testnet" || caller_id == contract_id,
            "You not have permission to remove admin rights"
        );
        assert!(
            self.is_admin(account_id.clone()),
            "This account not have admin rights"
        );
        self.admins.remove(&account_id)
    }

    /// Change state of contract, Only can be called by owner.
    #[payable]
    pub fn change_state(&mut self, state: RunningState) {
        assert_one_yocto();

        if self.app_config.running_state != state {
            if state == RunningState::Running {
                // only owner can resume the contract
                assert!(
                    self.is_admin(env::predecessor_account_id()),
                    "You don't have this permission"
                );
            }
            env::log(
                format!(
                    "Contract state changed from {} to {} by {}",
                    self.app_config.running_state,
                    state,
                    env::predecessor_account_id()
                )
                .as_bytes(),
            );
            self.app_config.running_state = state;
        }
    }

    pub fn approve_report(&mut self, report_id: ReportId) {
        let mut report = self.reports.get(&report_id).expect("Report not found");
    }

    pub fn reject_report(&mut self, report_id: ReportId) {
        let mut report = self.reports.get(&report_id).expect("Report not found");
        assert!(report.status == ReportStatus::Pending, "Cann't approved this report");
        report.status = ReportStatus::Rejected;
        self.reports.insert(&report_id, &report);
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
