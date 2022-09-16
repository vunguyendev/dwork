use super::*;

#[ext_contract(ext_self)]
pub trait ExtDupwork {
    fn on_transferd(
        &mut self,
        task_id: String,
        beneficiary_id: AccountId,
        amount_to_transfer: Balance,
    ) -> bool;
}

#[near_bindgen]
impl Dwork {
    // Ext
    // pub fn on_refund(
    //     &mut self,
    //     task_id: TaskId,
    //     owner_id: AccountId,
    //     amount_to_transfer: Balance,
    // ) -> bool {
    //     assert!(
    //         env::predecessor_account_id() == env::current_account_id(),
    //         "Callback is not called from the contract itself",
    //     );
    //
    //     assert!(
    //         env::promise_results_count() == 1,
    //         "Function called not as a callback",
    //     );
    //
    //     match env::promise_result(0) {
    //         PromiseResult::Successful(_) => {
    //             let mut owner = self.users.get(&owner_id).expect("Not found owner");
    //             owner.completed_jobs.insert(&task_id);
    //             owner.current_jobs.remove(&task_id);
    //
    //             if let UserType::Requester {
    //                 total_transfered,
    //                 current_requests,
    //             } = owner.user_type
    //             {
    //                 assert!(current_requests > 0, "Current requests is zero!");
    //                 owner.user_type = UserType::Requester {
    //                     total_transfered: total_transfered + amount_to_transfer,
    //                     current_requests: current_requests - 1,
    //                 };
    //
    //                 self.users.insert(&owner_id, &owner);
    //             }
    //
    //             true
    //         }
    //         _ => false,
    //     }
    // }

    pub fn on_transferd(
        &mut self,
        task_id: String,
        beneficiary_id: AccountId,
        amount_to_transfer: Balance,
    ) -> bool {
        assert!(
            env::predecessor_account_id() == env::current_account_id(),
            "Callback is not called from the contract itself",
        );

        assert!(
            env::promise_results_count() == 1,
            "Function called not as a callback",
        );

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut task = self.task_recores.get(&task_id).expect("Job not exist");
                let mut proposal = task
                    .proposals
                    .get(&beneficiary_id)
                    .expect("Proposal not found!");

                proposal.status = ProposalStatus::Approved;
                task.proposals.insert(&beneficiary_id, &proposal);
                self.task_recores.insert(&task_id, &task);

                let mut worker = self.users.get(&beneficiary_id).expect("Not found worker");
                worker.completed_jobs.insert(&task_id);
                worker.current_jobs.remove(&task_id);

                if task
                    .proposals
                    .iter()
                    .filter(|(_k, v)| {v.status == ProposalStatus::Approved})
                    .count() as u16
                    == task.max_participants
                {
                    let owner_id = task.owner;
                    let mut owner = self.users.get(&owner_id).expect("Not found owner");
                    owner.completed_jobs.insert(&task_id);
                    owner.current_jobs.remove(&task_id);
                    owner.total_spent += task.price * task.max_participants as u128;

                    self.users.insert(&owner_id, &owner);
                }

                worker.current_jobs.remove(&task_id);
                worker.completed_jobs.insert(&task_id);
                worker.total_earn += amount_to_transfer;
                self.users.insert(&beneficiary_id, &worker);
                true
            }
            _ => false,
        }
    }
}
