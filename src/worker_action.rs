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

        let mut task = self.task_recores.get(&task_id).expect("Job not exist");

        assert!(
            task.available_until > env::block_timestamp(),
            "This request is expire!"
        );

        assert!(
            task.proposals
                .iter()
                .filter(|(_k, v)| v.status == ProposalStatus::Approved)
                .count()
                < task.max_participants as usize,
            "Full approved participants"
        );

        //TODO increase worker current task
        let worker_id = env::predecessor_account_id();
        let mut worker = self.accounts.get(&worker_id).expect("User not found");
        worker.current_jobs.insert(&task_id);

        let proposal = Proposal {
            account_id: worker_id.clone(),
            proof_of_work: proof,
            status: ProposalStatus::Pending,
        };

        task.proposals.insert(&worker_id, &proposal);
        self.task_recores.insert(&task_id, &task);
    }

    #[payable]
    pub fn report_rejection (&mut self, task_id: String, report: String) {
        // Condition:
        //  - Owner of this submittion
        let mut task = self.task_recores.get(&task_id).expect("Task not exists");

        let worker_id = env::predecessor_account_id();

        let mut proposal = task.proposals.get(&worker_id).expect("Proposal not exists");
        
        match proposal.status {
            ProposalStatus::Rejected {reason} => {
                assert!(reason != "late", "Cannot report this reject reason");
                let report_id = worker_id.clone() + "_" + &task_id;
                let report = Report {
                    report_id: report_id.clone(),
                    account_id: worker_id.clone(),
                    task_id: task_id.clone(),
                    report,
                    status: ReportStatus::Pending
                };
                
                self.reports.insert(&report_id, &report);
                proposal.status = ProposalStatus::Reported { report_id };
                task.proposals.insert(&worker_id, &proposal);
                self.task_recores.insert(&task_id, &task);
            }
            ProposalStatus::Approved => panic!("Proposal have been approved"),
            _ => panic!("Proposal is not rejected")
        }
    }
}
