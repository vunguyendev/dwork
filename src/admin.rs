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
    pub report_interval: Timestamp,
    pub minimum_reward_per_task: Balance,
    pub maximum_reward_per_task: Balance,
    pub maximum_description_length: u16,
    pub maximum_cover_letter_length: u16,
    pub maximum_proposals_at_one_time: u16,
    pub maximum_requests_active_per_user: u16,
    pub maximum_title_length: u16,

    pub claim_point_bonus: u32, // may be a near bonus was given by requester to pay for who call
                                // first claim / complete task
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            running_state: RunningState::Running,
            register_bond: 500_000_000_000_000_000_000_000,
            submit_bond: 10_000_000_000_000_000_000_000,
            report_interval: 172_800_000_000_000, // 2 days
            minimum_reward_per_task: 10_000_000_000_000_000_000_000,
            maximum_reward_per_task: 100_000_000_000_000_000_000_000_000,
            maximum_description_length: 10000,
            maximum_cover_letter_length: 10000,
            maximum_proposals_at_one_time: 200,
            maximum_requests_active_per_user: 10,
            maximum_title_length: 100,

            claim_point_bonus: 10,
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
        assert!(
            report.status == ReportStatus::Pending,
            "Cann't approved this report"
        );

        let mut task = self.internal_get_task(&report.task_id);
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(report.task_id.clone(), report.account_id.clone());

        report.status = ReportStatus::Approved;
        self.reports.insert(&report_id, &report);

        proposal.status = ProposalStatus::Approved;
        self.proposals.insert(&proposal_id, &proposal);

        task.approved.push(report.account_id.clone());
        self.task_recores.insert(&report.task_id, &task);
    }

    pub fn reject_report(&mut self, report_id: ReportId) {
        let mut report = self.reports.get(&report_id).expect("Report not found");
        assert!(
            report.status == ReportStatus::Pending,
            "Cann't reject this report"
        );

        let mut task = self.internal_get_task(&report.task_id);
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(report.task_id.clone(), report.account_id.clone());

        report.status = ReportStatus::Rejected;
        self.reports.insert(&report_id, &report);

        proposal.status = ProposalStatus::Rejected {
            reason: "Completed".to_string(),
        };
        self.proposals.insert(&proposal_id, &proposal);

        task.approved.push(report.account_id.clone());
        self.task_recores.insert(&report.task_id, &task);
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
    //

    pub fn check_available_review_report(&self, task_id: TaskId) -> bool {
        let task = self.task_recores.get(&task_id).expect("Task not found");
        if let Some(end_review) = task.review_proposal_complete_at {
            let now = env::block_timestamp();
            if now > end_review + self.app_config.report_interval {
                return task
                    .proposals
                    .iter()
                    .filter(|v| {
                        let proposal = self.proposals.get(v).expect("Proposal not found");
                        match proposal.status {
                            ProposalStatus::Reported { report_id } => {
                                let report =
                                    self.reports.get(&report_id).expect("Report not found");
                                report.status == ReportStatus::Pending
                            }
                            _ => false,
                        }
                    })
                    .count()
                    > 0;
            }
        }
        true
    }

    pub fn mark_task_as_completed(&mut self, _task_id: TaskId) {
        // let task = self.task_recores.get(&task_id).expect("Task not found");
        //
        // let beneficiary_id = env::predecessor_account_id();
        // assert!(
        //     task.owner == beneficiary_id,
        //     "Only owner can reject proposal"
        // );
        //
        // assert!(
        //     task.proposals
        //         .iter()
        //         .filter(|(_k, v)| v.status == ProposalStatus::Pending)
        //         .count()
        //         == 0
        //         || task.approved.len() == task.max_participants as usize,
        //     "Some work remains unchecked"
        // );
        //
        // let completed_proposals_count = task
        //     .proposals
        //     .iter()
        //     .filter(|(_k, v)| v.status == ProposalStatus::Approved)
        //     .count();
        //
        // let refund: u64 = (task.max_participants as u64) - task.proposals.len();
        //
        // let amount_to_transfer = (task.price as u128)
        //     .checked_mul(refund.into())
        //     .expect("Can not calculate amount to refund");
        // if completed_proposals_count < task.max_participants as usize {
        //     assert!(
        //         task.submit_available_until < env::block_timestamp(),
        //         "This request is not expire, you can not mark it completed!"
        //     );
        //
        //     Promise::new(beneficiary_id.to_string()).transfer(amount_to_transfer);
        // }
        //
        // let mut owner = self.accounts.get(&beneficiary_id).expect("Not found owner");
        // owner.completed_jobs.insert(&task_id);
        // owner.current_jobs.remove(&task_id);
        // owner.total_spent += task.price * task.max_participants as u128 - amount_to_transfer;
        // self.accounts.insert(&beneficiary_id, &owner);
    }
}
