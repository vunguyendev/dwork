use crate::*;

pub type TaskId = String;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Task {
    pub owner: AccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub price: Balance,
    pub buget: Balance,
    pub proposals: Vec<ProposalId>,
    pub created_at: Timestamp,
    pub last_rejection_published_at: Option<Timestamp>,
    pub submit_available_until: Timestamp,
    pub category_id: CategoryId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTask {
    pub owner: AccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub price: WrappedBalance,
    pub proposals: Vec<Proposal>,
    pub created_at: WrappedTimestamp,
    pub available_until: WrappedTimestamp,
    pub category_id: CategoryId,
}

// impl From<Task> for WrappedTask {
impl Dwork {
    pub fn json_from_task(&self, task: Task) -> WrappedTask {
        let Task {
            owner,
            title,
            description,
            max_participants,
            price,
            buget: _,
            proposals,
            created_at,
            last_rejection_published_at: _,
            submit_available_until,
            category_id,
        } = task;
        
        let proposals: Vec<Proposal> = proposals
            .iter()
            .map(|item| self.proposals.get(item).expect("Proposal not found"))
            .collect();

        WrappedTask {
            owner,
            title,
            description,
            max_participants,
            price: WrappedBalance::from(price),
            proposals,
            created_at: WrappedTimestamp::from(created_at),
            available_until: WrappedTimestamp::from(submit_available_until),
            category_id,
        }
    }
}

#[near_bindgen]
impl Dwork {
    pub(crate) fn internal_get_task(&self, task_id: &TaskId) -> Task {
        self.task_recores.get(task_id).expect("Task not found")
    }

    pub(crate) fn internal_gen_proposal_id(&self, task_id: TaskId, worker_id: AccountId) -> String {
        task_id + "_" + &worker_id
    }

    pub(crate) fn internal_get_proposal(
        &self,
        task_id: TaskId,
        worker_id: String,
    ) -> (ProposalId, Proposal) {
        let task = self.internal_get_task(&task_id);
        let proposal_id = self.internal_gen_proposal_id(task_id, worker_id);
        assert!(task.proposals.contains(&proposal_id), "Invalid proposal id");
        (
            proposal_id.clone(),
            self.proposals
                .get(&proposal_id)
                .expect("Proposal not found"),
        )
    }

    pub fn available_tasks(&self, from_index: u64, limit: u64) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self.task_recores.keys_as_vector();

        calculate_rev_limit(tasks_id.len(), from_index, limit)
            .map(|index| {
                let task_id = tasks_id.get(index as u64).unwrap();
                let task = self.task_recores.get(&task_id).unwrap();
                (task_id, self.json_from_task(task))
            })
            .rev()
            .collect()
    }

    pub fn current_tasks(
        &self,
        account_id: AccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self
            .accounts
            .get(&account_id)
            .expect("User not found")
            .current_jobs
            .to_vec();

        calculate_rev_limit(tasks_id.len() as u64, from_index, limit)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    self.json_from_task(self.internal_get_task(key)),
                )
            })
            .rev()
            .collect()
    }

    pub fn completed_tasks(
        &self,
        account_id: AccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self
            .accounts
            .get(&account_id)
            .expect("User not found")
            .completed_jobs
            .to_vec();

        calculate_rev_limit(tasks_id.len() as u64, from_index, limit)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    self.json_from_task(self.internal_get_task(key)),
                )
            })
            .rev()
            .collect()
    }

    pub fn task_by_id(&self, task_id: TaskId) -> WrappedTask {
        self.task_recores
            .get(&task_id)
            .map(|task| self.json_from_task(task))
            .expect("Task not found")
    }

    pub fn tasks_by_ids(&self, ids: Vec<String>) -> Vec<(String, WrappedTask)> {
        ids.iter()
            .map(|id| (id.clone(), self.task_by_id(id.to_string())))
            .collect()
    }

    pub fn maximum_participants_per_task(&self) -> u16 {
        self.app_config.maximum_proposals_at_one_time
    }
}
