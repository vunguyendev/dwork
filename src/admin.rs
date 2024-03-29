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
    pub validate_report_interval: Timestamp,

    pub minimum_reward_per_task: Balance,
    pub maximum_reward_per_task: Balance,
    pub maximum_description_length: u16,
    pub maximum_cover_letter_length: u16,
    pub maximum_proposals_at_one_time: u16,
    pub maximum_requests_active_per_user: u16,
    pub maximum_title_length: u16,
    
    pub minimum_deposit: Balance,
    pub maximum_deposit: Balance,

    pub big_plus: u16,
    pub med_plus: u16,
    pub sml_plus: u16,

    pub big_minus: u16,
    pub med_minus: u16,
    pub sml_minus: u16,
    
    pub claim_point_bonus: u32, // may be a near bonus was given by requester to pay for who call
                                // first claim / complete task
    pub critical_point: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            running_state: RunningState::Running,
            register_bond: 500_000_000_000_000_000_000_000,
            submit_bond: 10_000_000_000_000_000_000_000,
            report_interval: 172_800_000_000_000, // 2 days
            validate_report_interval: 259_200_000_000_000, // 3 days
            minimum_reward_per_task: 10_000_000_000_000_000_000_000,
            maximum_reward_per_task: 100_000_000_000_000_000_000_000_000,
            maximum_description_length: 10000,
            maximum_cover_letter_length: 10000,
            maximum_proposals_at_one_time: 200,
            maximum_requests_active_per_user: 10,
            maximum_title_length: 100,

            minimum_deposit: 100_000_000_000_000_000_000_000, // 0.1 N
            maximum_deposit: 500_000_000_000_000_000_000_000_000, // 500 N

            claim_point_bonus: 10,
            critical_point: 85,
            
            big_plus: 15,
            med_plus: 10,
            sml_plus: 5,

            big_minus: 20,
            med_minus: 15,
            sml_minus: 10,
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
        assert!(self.is_admin(env::predecessor_account_id()), "For now, just admin can approve report");
        assert!(
            report.status == ReportStatus::Pending,
            "Can't approved this report"
        );

        let task = self.internal_get_task(&report.task_id);
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(report.task_id.clone(), report.account_id.clone());

        match proposal.status {
            ProposalStatus::Rejected {
                reason: _,
                reject_at: _,
                report_id,
            } => {
                assert!(report_id.is_none(), "Invalid report");
            }
            _ => panic!("Invalid report"),
        }

        // Update Report status
        report.status = ReportStatus::Approved;
        self.reports.insert(&report_id, &report);

        // Update Proposal Status
        proposal.status = ProposalStatus::ApprovedByAdmin{account_id: env::predecessor_account_id()};
        self.proposals.insert(&proposal_id, &proposal);

        /* Update Worker Locked balance
         * - Add Locked Balance for this worker
         * - Remove Locked Balance from last worker (if needed)
         */
        // Add locked balance for woker
        let mut worker = self.internal_get_account(&report.account_id);
        let mut owner = self.internal_get_account(&task.owner);
        let release_at: Timestamp = match task.last_rejection_published_at {
            Some(time) => {
                time + self.app_config.report_interval + self.app_config.validate_report_interval
            }
            None => env::block_timestamp(),
        };
        let new_locked_balance = LockedBalance {
            amount: task.price,
            release_at,
            // Must be the last rejection deadline report + 3 days
        };
        
        worker.add_pos_point(self.app_config.sml_plus as u32);
        worker.locked_balance.insert(&report.task_id, &new_locked_balance);
        self.internal_set_account(&report.account_id, worker);

        // BIG minus for wrong rejection
        owner.add_neg_point(self.app_config.big_minus as u32);
        self.internal_set_account(&task.owner, owner);
        
        let mut num_approvals = 0;
        for proposal_id in task.proposals.iter() {
            let mut proposal = self
                .proposals
                .get(proposal_id)
                .expect("Proposal not found");

            match proposal.status {
                ProposalStatus::Approved => {
                    if num_approvals >= task.max_participants {
                        proposal.status = ProposalStatus::Rejected {
                            reason: "late".to_string(),
                            reject_at: env::block_timestamp(),
                            report_id: None,
                        };
                        // Remove locked balance
                        let mut worker = self.internal_get_account(&proposal.account_id);
                        worker.locked_balance.remove(&report.task_id);
                    } else {
                        num_approvals += 1;
                    }
                }
                ProposalStatus::Pending => break,
                _ => {}
            }
        }
    }

    pub fn reject_report(&mut self, report_id: ReportId) {
        let mut report = self.reports.get(&report_id).expect("Report not found");
        assert!(self.is_admin(env::predecessor_account_id()), "For now, just admin can reject report");
        assert!(
            report.status == ReportStatus::Pending,
            "Cann't reject this report"
        );

        let mut worker = self.internal_get_account(&report.account_id);
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(report.task_id.clone(), report.account_id.clone());
        
        match proposal.status {
            ProposalStatus::Rejected {
                reason: _,
                reject_at: _,
                report_id,
            } => {
                assert!(report_id.is_none(), "Invalid report");
            }
            _ => panic!("Invalid report"),
        }

        worker.add_neg_point(self.app_config.med_minus as u32);
        self.internal_set_account(&report.account_id, worker);

        report.status = ReportStatus::Rejected;
        self.reports.insert(&report_id, &report);

        proposal.status = ProposalStatus::RejectedByAdmin {account_id: env::predecessor_account_id()};
        self.proposals.insert(&proposal_id, &proposal);
    }
}
