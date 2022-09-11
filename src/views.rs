use crate::*;
use near_sdk::serde_json::{json, Value};

#[near_bindgen]
impl Dwork {
    pub fn available_tasks(&self, from_index: u64, limit: u64) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self.task_recores.keys_as_vector();

        let from = if tasks_id.len() > (limit + from_index) {
            tasks_id.len() - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() > from_index {
            tasks_id.len() - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let task_id = tasks_id.get(index as u64).unwrap();
                let task = self.task_recores.get(&task_id).unwrap();
                (task_id, WrappedTask::from(task))
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
            .users
            .get(&account_id)
            .expect("User not found")
            .current_jobs
            .to_vec();

        let from = if tasks_id.len() as u64 > (limit + from_index) {
            tasks_id.len() as u64 - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() as u64 > from_index {
            tasks_id.len() as u64 - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    WrappedTask::from(self.task_recores.get(key).unwrap()),
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
            .users
            .get(&account_id)
            .expect("User not found")
            .completed_jobs
            .to_vec();

        let from = if tasks_id.len() as u64 > (limit + from_index) {
            tasks_id.len() as u64 - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() as u64 > from_index {
            tasks_id.len() as u64 - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    WrappedTask::from(self.task_recores.get(key).unwrap()),
                )
            })
            .rev()
            .collect()
    }

    pub fn user_info(&self, account_id: AccountId) -> Value {
        self.users
            .get(&account_id)
            .map(|v| {
                json!({
                    "account_id": v.account_id,
                    "bio": v.bio,
                    "completed_jobs": v.completed_jobs.to_vec(),
                    "current_jobs": v.current_jobs.to_vec(),
                    "total_earn": v.total_earn,
                    "total_spent": v.total_spent,
                    "locked_balance": v.locked_balance
                })
            })
            .expect("Canot map user to json")
    }

    pub fn task_by_id(&self, task_id: TaskId) -> WrappedTask {
        self.task_recores
            .get(&task_id)
            .map(WrappedTask::from)
            .expect("Task not found")
    }

    pub fn tasks_by_ids(&self, ids: Vec<String>) -> Vec<(String, WrappedTask)> {
        ids.iter()
            .map(|id| (id.clone(), self.task_by_id(id.to_string())))
            .collect()
    }

    //Get categories
    pub fn categories(&self, from_index: u64, limit: u64) -> Vec<Category> {
        let category_ids = self.categories.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, category_ids.len()))
            .map(|index| {
                let category_id = category_ids.get(index).unwrap();
                self.categories.get(&category_id).unwrap()
            })
            .collect()
    }

    pub fn maximum_participants_per_task(&self) -> u16 {
        self.app_config.maximum_proposals_at_one_time
    }
}
