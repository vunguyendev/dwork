use crate::*;
// use near_sdk::ONE_NEAR;

#[near_bindgen]
impl Dwork {
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
        let owner_id = env::predecessor_account_id();
        let mut owner = self.internal_get_account(&owner_id);

        // Validate storage deposit
        let storage_update = self.new_storage_update(owner_id.clone());

        let unwrap_balance: Balance = price.into();
        let mut amount_need_to_pay: Balance = (max_participants as u128)
            .checked_mul(unwrap_balance)
            .expect("Cannot calculate total amount");
        // Get 10% for other service
        amount_need_to_pay = amount_need_to_pay + amount_need_to_pay / 10;

        let mut category = self
            .categories
            .get(&category_id)
            .expect("Category not found");
        let task_id = env::predecessor_account_id() + "_" + &env::block_index().to_string();

        assert!(
            self.task_recores.get(&task_id).is_none(),
            "Can't post twice per block"
        );

        assert!(
            amount_need_to_pay >= self.app_config.minimum_reward_per_task
                && amount_need_to_pay <= self.app_config.maximum_reward_per_task,
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
            "Only accept {} participants at one time",
            self.app_config.maximum_proposals_at_one_time
        );

        // Using balance to create task
        // self.internal_payment(amount_need_to_pay);
        assert!(
            env::attached_deposit() == amount_need_to_pay,
            "You must attach exactly {} yocto near to create task",
            amount_need_to_pay / 1_000_000_000_000_000_000_000_000
        );

        let unwrap_duration: Duration = duration.into();

        let task = Task {
            owner: owner_id.clone(),
            title,
            description,
            price: price.into(),
            max_participants,
            buget: amount_need_to_pay,
            proposals: Vec::new(),
            created_at: env::block_timestamp(),
            submit_available_until: env::block_timestamp() + unwrap_duration,
            category_id: category_id.clone(),
            last_rejection_published_at: None,
        };

        //Update num_posts in category
        category.num_posts += 1;
        self.categories.insert(&category_id, &category);

        // Add task to task recores
        self.task_recores.insert(&task_id, &task);

        // Update Owner Account
        owner.current_jobs.insert(&task_id);
        self.internal_set_account(&owner_id, owner);

        self.finalize_storage_update(storage_update);
    }

    pub fn approve_work(&mut self, task_id: TaskId, worker_id: AccountId) {
        let storage_update = self.new_storage_update(worker_id.clone());

        // Check task condition
        let task = self.internal_get_task(&task_id);
        assert!(
            task.proposals
                .iter()
                .filter(|proposal_id| self
                    .proposals
                    .get(proposal_id)
                    .expect("Proposal not found")
                    .status
                    == ProposalStatus::Approved)
                .count()
                < task.max_participants.into(),
            "You have approved for {} participants",
            task.max_participants
        );

        assert!(
            task.owner == env::predecessor_account_id(),
            "Only owner can approve proposal"
        );

        // Check proposal condition
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(task_id.clone(), worker_id.clone());

        assert!(
            proposal.status == ProposalStatus::Pending,
            "You already approved or rejected this worker!!"
        );

        // Update proposal
        proposal.status = ProposalStatus::Approved;
        self.proposals.insert(&proposal_id, &proposal);

        // Set locked balance for worker
        let mut worker = self.internal_get_account(&worker_id);
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
        worker.locked_balance.insert(&task_id, &new_locked_balance);
        worker.add_pos_point(self.app_config.sml_plus as u32);
        self.internal_set_account(&worker_id, worker);

        self.finalize_storage_update(storage_update);
    }

    //TODO: add reason by owner CHECKED
    pub fn reject_work(&mut self, task_id: TaskId, worker_id: AccountId, reason: String) {
        let storage_update = self.new_storage_update(worker_id.clone());

        // Check task condition
        let mut task = self.internal_get_task(&task_id);

        assert!(
            task.owner == env::predecessor_account_id(),
            "Only owner can approve proposal"
        );

        // Check proposal condition
        let (proposal_id, mut proposal) =
            self.internal_get_proposal(task_id.clone(), worker_id);

        assert!(
            proposal.status == ProposalStatus::Pending,
            "You already approved or rejected this worker!!"
        );

        // Update proposal
        proposal.status = ProposalStatus::Rejected {
            reason,
            reject_at: env::block_timestamp(),
            report_id: None,
        };
        self.proposals.insert(&proposal_id, &proposal);

        // Update task
        task.last_rejection_published_at = Some(env::block_timestamp());
        self.task_recores.insert(&task_id, &task);

        self.finalize_storage_update(storage_update);
    }

    // Will refund remainder amount for owner
    // Just task owner can call this function
    pub fn mark_task_as_completed(&mut self, task_id: TaskId) {
        let storage_update = self.new_storage_update(env::predecessor_account_id());

        let task = self.internal_get_task(&task_id);
        let mut owner = self.internal_get_account(&task.owner);

        assert_eq!(
            task.owner,
            env::predecessor_account_id(),
            "Only owner can mark this task as complete"
        );

        if task.proposals.is_empty() {
            assert!(
                task.submit_available_until > env::block_timestamp(),
                "Wait until end submit interval"
            )
        } else {
            assert!(
                task.proposals
                    .iter()
                    .filter(|proposal_id| {
                        match self
                            .proposals
                            .get(proposal_id)
                            .expect("Proposal not found")
                            .status
                        {
                            ProposalStatus::Rejected {
                                reason: _,
                                reject_at: _,
                                report_id,
                            } => report_id.is_some(),
                            _ => false,
                        }
                    })
                    .count()
                    == 0,
                "Validating reports!"
            );

            assert!(
                task.last_rejection_published_at.is_none()
                    || task.last_rejection_published_at.unwrap() + self.app_config.report_interval
                        < env::block_timestamp(),
                "Now is in report interval."
            );
        }

        let reports_by = task
            .proposals
            .iter()
            .filter_map(|proposal_id| {
                (matches!(
                    self.proposals
                        .get(proposal_id)
                        .expect("Proposal not found")
                        .status,
                    ProposalStatus::ApprovedByAdmin { account_id: _ }
                        | ProposalStatus::RejectedByAdmin { account_id: _ }
                ))
                .then(|| {
                    match self
                        .proposals
                        .get(proposal_id)
                        .expect("Proposal not found")
                        .status
                    {
                        ProposalStatus::ApprovedByAdmin { account_id } => account_id,
                        ProposalStatus::RejectedByAdmin { account_id } => account_id,
                        _ => "".to_string(),
                    }
                })
            })
            .collect::<Vec<String>>();

        let refund: u64 = task
            .proposals
            .iter()
            .filter(|proposal_id| {
                self.proposals
                    .get(proposal_id)
                    .expect("Proposal not found")
                    .status
                    == ProposalStatus::Approved
            })
            .count() as u64;

        let mut remainder = task.buget
            - (task.price)
                .checked_mul(refund.into())
                .expect("Can not calculate amount to refund");

        if !reports_by.is_empty() {
            let amount = remainder / (reports_by.len() as u128);
            reports_by
                .iter()
                .for_each(|account_id| self.internal_send(Some(account_id.to_string()), amount));
            remainder = 0;
        }

        owner.completed_jobs.insert(&task_id);
        owner.current_jobs.remove(&task_id);
        owner.total_spent += task.buget - remainder;
        owner.add_pos_point(self.app_config.sml_plus as u32);
        self.internal_set_account(&task.owner, owner);
        self.internal_send(None, remainder);

        self.finalize_storage_update(storage_update);
    }
}
