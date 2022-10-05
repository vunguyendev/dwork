use crate::*;

#[near_bindgen]
impl Dwork {
    #[payable]
    pub fn submit_work(&mut self, task_id: String, proof: String) {
        // TODO: Allow user to use current balance
        assert!(
            env::attached_deposit() == self.app_config.submit_bond,
            "Send exactly {:?} Near to submit",
            self.app_config.submit_bond
        );

        let mut task = self.internal_get_task(&task_id);
        let worker_id = env::predecessor_account_id();
        let now = env::block_timestamp();
        let proposal_id = self.internal_gen_proposal_id(task_id.clone(), worker_id.clone());

        assert!(task.submit_available_until > now, "Request is expired");

        assert!(self.proposals.get(&proposal_id).is_none(), "Already submitted this task");

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

        // Increase worker current task
        let mut worker = self.internal_get_account(&worker_id);
        worker.current_jobs.insert(&task_id);
        self.internal_set_account(&worker_id, worker);

        let proposal = Proposal {
            account_id: worker_id,
            submit_time: now,
            proof_of_work: proof,
            status: ProposalStatus::Pending,
        };

        self.proposals.insert(&proposal_id, &proposal);

        task.proposals.push(proposal_id);
        self.task_recores.insert(&task_id, &task);
    }

    #[payable]
    pub fn report_rejection(&mut self, task_id: String, report: String) {
        let worker_id = env::predecessor_account_id();
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(task_id.clone(), worker_id.clone());

        match proposal.status {
            ProposalStatus::Rejected {
                reason,
                reject_at,
                report_id,
            } => {
                assert!(reason != "late", "Cannot report this reject reason");
                assert!(report_id.is_none(), "Reported this rejection");
                assert!(
                    reject_at + self.app_config.report_interval > env::block_timestamp(),
                    "Not available to report this rejection"
                );

                // Update reports
                let report_id = worker_id.clone() + "_" + &task_id;
                let report = Report {
                    report_id: report_id.clone(),
                    account_id: worker_id,
                    task_id,
                    report,
                    status: ReportStatus::Pending,
                };

                self.reports.insert(&report_id, &report);
                
                // Update proposal
                proposal.status = ProposalStatus::Rejected {
                    reason,
                    reject_at,
                    report_id: Some(report_id),
                };
                self.proposals.insert(&proposal_id, &proposal);
            }
            ProposalStatus::Approved => panic!("Proposal have been approved"),
            _ => panic!("Proposal is not rejected"),
        }
    }

    pub fn claim(&mut self, task_id: TaskId) {
        // let mut task = self.internal_get_task(&task_id);
        let worker_id = env::predecessor_account_id();
        let mut worker = self.internal_get_account(&worker_id);
        let LockedBalance { amount, release_at } = worker.locked_balance.get(&task_id).expect("Locked Balance not found");
        
        assert!(release_at < env::block_timestamp(), "This balance still be locked");

        worker.add_pos_point(self.app_config.sml_plus as u32);
        worker.locked_balance.remove(&task_id);
        self.internal_send(None, amount);
        self.internal_set_account(&worker_id, worker);
    }
}
