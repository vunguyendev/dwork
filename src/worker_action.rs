use crate::*;

#[near_bindgen]
impl Dwork {
    #[payable]
    pub fn submit_work(&mut self, task_id: String, proof: String) {
        assert!(
            env::attached_deposit() == self.app_config.submit_bond,
            "Send exactly {:?} Near to submit",
            self.app_config.submit_bond
        );

        let mut task = self.internal_get_task(task_id);

        assert!(
            task.submit_available_until > env::block_timestamp(),
            "This request is expire!"
        );

        assert!(
            task.proposals
                .iter()
                .filter(
                    |v| self.proposals.get(v).expect("Proposal not found").status
                        == ProposalStatus::Approved
                )
                .count()
                < task.max_participants as usize,
            "Full approved participants"
        );

        //TODO increase worker current task
        let worker_id = env::predecessor_account_id();
        let mut worker = self.internal_get_account(&worker_id);
        let proposal_id = task_id + "_" + &worker_id.to_string();

        worker.current_jobs.insert(&task_id);

        let proposal = Proposal {
            account_id: worker_id.clone(),
            submit_time: env::block_timestamp(),
            proof_of_work: proof,
            status: ProposalStatus::Pending,
        };

        self.proposals.insert(&proposal_id, &proposal);
        task.proposals.push(proposal_id);
    
        self.task_recores.insert(&task_id, &task);
        self.internal_set_account(&worker_id, worker);
    }

    #[payable]
    pub fn report_rejection(&mut self, task_id: String, report: String) {
        let mut task = self.task_recores.get(&task_id).expect("Task not exists");
        assert!(
            task.review_proposal_complete_at.is_none()
                || task.review_proposal_complete_at.unwrap() + self.app_config.report_interval
                    > env::block_timestamp(),
            "Time for report is expired"
        );

        let worker_id = env::predecessor_account_id();

        let mut proposal = task.proposals.get(&worker_id).expect("Proposal not exists");

        match proposal.status {
            ProposalStatus::Rejected { reason } => {
                assert!(reason != "late", "Cannot report this reject reason");
                let report_id = worker_id.clone() + "_" + &task_id;
                let report = Report {
                    report_id: report_id.clone(),
                    account_id: worker_id.clone(),
                    task_id: task_id.clone(),
                    report,
                    status: ReportStatus::Pending,
                };

                self.reports.insert(&report_id, &report);

                proposal.status = ProposalStatus::Reported { report_id };
                task.proposals.insert(&worker_id, &proposal);

                self.task_recores.insert(&task_id, &task);
            }
            ProposalStatus::Approved => panic!("Proposal have been approved"),
            _ => panic!("Proposal is not rejected"),
        }
    }

    pub fn claim(&mut self, task_id: TaskId) {
        let mut task = self.task_recores.get(&task_id).expect("Task not found");
        let mut caller = self.internal_get_account(&env::predecessor_account_id());

        assert!(
            !self.check_available_review_report(task_id.clone()),
            "Task still in process"
        );

        if task.approved.len() > task.max_participants as usize {
            let mut approvals: Vec<Proposal> = task
                .approved
                .clone()
                .iter()
                .map(|v| task.proposals.get(v).expect("Proposal not found"))
                .collect();
            approvals.sort_by(|a, b| a.submit_time.partial_cmp(&b.submit_time).unwrap());
            let valid_approvals = &approvals[..task.max_participants as usize];
            let final_approve: Vec<AccountId> = valid_approvals
                .iter()
                .map(|v| v.account_id.clone())
                .collect();
            task.approved = final_approve;
            self.task_recores.insert(&task_id, &task);

            caller.pos_point += self.app_config.claim_point_bonus;
            self.internal_set_account(&env::predecessor_account_id(), caller);
        }
        let mut worker = self.internal_get_account(&env::predecessor_account_id());
        let amount = task.price;
        worker.balance += amount + self.app_config.submit_bond;
    }
}
