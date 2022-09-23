use crate::*;

#[near_bindgen]
impl Dwork {
    /// Requester sections:
    #[payable]
    pub fn new_task(
        &mut self,
        title: String,
        description: String,
        price: WrappedBalance,
        max_participants: u16,
        duration: WrappedDuration,
        category_id: CategoryId,
    ) {
        //TODO: Maximum deposit
        let owner = env::predecessor_account_id();
        let mut user = self.internal_get_account(&owner);

        let unwrap_balance: Balance = price.into();
        let amount_need_to_pay: Balance = (max_participants as u128)
            .checked_mul(unwrap_balance)
            .expect("Cannot calculate total amount");

        assert!(
            env::attached_deposit() == amount_need_to_pay,
            "Attach exactly {} yoctoNear",
            amount_need_to_pay
        );

        assert!(
            env::attached_deposit() >= self.app_config.minimum_reward_per_task
                && env::attached_deposit() <= self.app_config.maximum_reward_per_task,
            "Total amount for each task must be in a range from {} to {}",
            self.app_config.minimum_reward_per_task,
            self.app_config.maximum_reward_per_task
        );

        assert!(
            description.len() <= self.app_config.maximum_description_length.into(),
            "Description too long"
        );

        assert!(
            max_participants <= self.app_config.maximum_proposals_at_one_time,
            "Only accept {} at one time",
            self.app_config.maximum_proposals_at_one_time
        );

        let storage_update = self.new_storage_update(owner.clone());
        let task_id = env::predecessor_account_id() + "_" + &env::block_index().to_string();

        assert!(
            self.task_recores.get(&task_id).is_none(),
            "Can't post twice per block"
        );

        let unwrap_duration: Duration = duration.into();

        let task = Task {
            owner: owner.clone(),
            title,
            description,
            price: price.into(),
            max_participants,
            proposals: Vec::new(), 
            approved: Vec::with_capacity(max_participants.into()),
            created_at: env::block_timestamp(),
            submit_available_until: env::block_timestamp() + unwrap_duration,
            review_proposal_complete_at: None,
            category_id: category_id.clone(),
        };

        //Update num_posts in category
        if let Some(mut category) = self.categories.get(&category_id) {
            category.num_posts += 1;
            self.categories.insert(&category_id, &category);
        } else {
            panic!("Not found your category");
        }

        self.task_recores.insert(&task_id, &task);
        user.balance += env::attached_deposit();
        user.current_jobs.insert(&task_id);
        self.accounts.insert(&owner, &user);

        self.finalize_storage_update(storage_update);
    }

    fn check_available_review_proposal(&mut self, task_id: TaskId) -> bool {
        let task = self.task_recores.get(&task_id).expect("Task not found");
        let now = env::block_timestamp();
        if now > task.submit_available_until {
            return task.approved.len() < task.max_participants as usize
                && task
                    .proposals
                    .iter()
                    .filter(|v| self.proposals.get(v).expect("Proposal not found").status == ProposalStatus::Pending)
                    .count()
                    > 0;
        }
        true
    }

    pub fn approve_work(&mut self, task_id: TaskId, worker_id: AccountId) {
        let mut task = self.task_recores.get(&task_id).expect("Task doesn't exist");

        let storage_update = self.new_storage_update(worker_id.clone());

        assert!(
            task.approved.len() < task.max_participants.into(),
            "You have approved for {} participants",
            task.max_participants
        );

        assert!(
            task.owner == env::predecessor_account_id(),
            "Only owner can approve proposal"
        );

        let proposal_id = self.internal_gen_proposal_id(task_id.clone(), worker_id.clone());
        assert!(task.proposals.contains(&proposal_id), "Invalid proposal_id");
        let mut proposal = self
            .proposals
            .get(&worker_id)
            .expect("Proposal doesn't found");
        // let beneficiary_id = proposal.account_id;
        // let amount_to_transfer = task.price;

        assert!(
            proposal.status != ProposalStatus::Pending,
            "You already approved or rejected this worker!!"
        );

        proposal.status = ProposalStatus::Approved;

        self.proposals.insert(&worker_id, &proposal);
        task.approved.push(worker_id.clone());

        self.task_recores.insert(&task_id, &task);

        if self.check_available_review_proposal(task_id.clone()) {
            self.mark_task_as_completed(task_id);
        }
        self.finalize_storage_update(storage_update);

        // Make a transfer to the worker
        // Promise::new(beneficiary_id.to_string())
        //     .transfer(amount_to_transfer + self.app_config.submit_bond)
        //     .then(ext_self::on_transferd(
        //         task_id,
        //         beneficiary_id,
        //         amount_to_transfer,
        //         &env::current_account_id(),
        //         0,
        //         DEFAULT_GAS_TO_PAY,
        //     ));
    }

    //TODO: add reason by owner CHECKED
    pub fn reject_work(&mut self, task_id: TaskId, worker_id: AccountId, reason: String) {
        let task = self.task_recores.get(&task_id).expect("Job not exist");

        let beneficiary_id = env::predecessor_account_id();
        assert!(
            task.owner == beneficiary_id,
            "Only owner can reject proposal"
        );

        let storage_update = self.new_storage_update(beneficiary_id);

        let mut proposal = self.proposals.get(&worker_id).expect("Not found proposal");
        assert!(
            proposal.status != ProposalStatus::Pending,
            "You already approved or rejected this worker!!"
        );

        proposal.status = ProposalStatus::Rejected { reason };

        self.proposals.insert(&worker_id, &proposal);
        self.task_recores.insert(&task_id, &task);

        if self.check_available_review_proposal(task_id.clone()) {
            self.mark_task_as_completed(task_id);
        }
        self.finalize_storage_update(storage_update)

        // let amount_to_transfer: Balance = task.price.into();
        // Promise::new(beneficiary_id.to_string()).transfer(amount_to_transfer + SUBMIT_BOND);
    }

    pub fn review_proposal_complete(&mut self, task_id: TaskId) {
        let now = env::block_timestamp();
        let mut task = self.task_recores.get(&task_id).expect("Task not found");
        task.review_proposal_complete_at = Some(now);

        self.task_recores.insert(&task_id, &task);
    }

    // pub fn mark_task_as_completed(&mut self, task_id: TaskId) {
    //     let task = self.task_recores.get(&task_id).expect("Job not exist");
    //
    //     let beneficiary_id = env::predecessor_account_id();
    //     assert!(
    //         task.owner == beneficiary_id,
    //         "Only owner can reject proposal"
    //     );
    //
    //     assert!(
    //         task.proposals
    //             .iter()
    //             .filter(|(_k, v)| v.status == ProposalStatus::Pending)
    //             .count()
    //             == 0
    //             || task.approved.len() == task.max_participants as usize,
    //         "Some work remains unchecked"
    //     );
    //
    //     let completed_proposals_count = task
    //         .proposals
    //         .iter()
    //         .filter(|(_k, v)| v.status == ProposalStatus::Approved)
    //         .count();
    //
    //     let refund: u64 = (task.max_participants as u64) - task.proposals.len();
    //
    //     let amount_to_transfer = (task.price as u128)
    //         .checked_mul(refund.into())
    //         .expect("Can not calculate amount to refund");
    //     if completed_proposals_count < task.max_participants as usize {
    //         assert!(
    //             task.submit_available_until < env::block_timestamp(),
    //             "This request is not expire, you can not mark it completed!"
    //         );
    //
    //         Promise::new(beneficiary_id.to_string()).transfer(amount_to_transfer);
    //     }
    //
    //     let mut owner = self.accounts.get(&beneficiary_id).expect("Not found owner");
    //     owner.completed_jobs.insert(&task_id);
    //     owner.current_jobs.remove(&task_id);
    //     owner.total_spent += task.price * task.max_participants as u128 - amount_to_transfer;
    //     self.accounts.insert(&beneficiary_id, &owner);
    // }
}
