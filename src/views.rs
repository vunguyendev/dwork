use crate::*;
use near_sdk::serde_json::{json, Value};

#[near_bindgen]
impl Dupwork {
    pub fn available_tasks(&self, from_index: u64, limit: u64) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self.tasks_recores.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, tasks_id.len() as u64))
            .map(|index| {
                let task_id = tasks_id.get(index as u64).unwrap();
                let task = self.tasks_recores.get(&task_id.clone()).unwrap();
                (task_id.clone(), task)
            })
            .filter(|(_k, v)| v.status == JobStatus::ReadyForApply)
            .map(|(k, task)| (k, WrappedTask::from(task)))
            .collect()
    }

    pub fn current_tasks(
        &self,
        account_id: ValidAccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self
            .users
            .get(&account_id)
            .expect("User not found")
            .current_jobs;

        tasks_id
            .iter()
            .map(|k| {
                (
                    k.clone(),
                    WrappedTask::from(self.tasks_recores.get(&k).unwrap()),
                )
            })
            .collect()
    }

    pub fn user_info(&self, account_id: ValidAccountId) -> Value {
        self.users
            .get(&account_id)
            .map(|v| {
                json!({
                    "account_id": v.account_id,
                    "user_type": WrappedUserType::from(v.user_type),
                    "completed_jobs": v.completed_jobs.to_vec()
                })
            })
            .expect("Canot map user to json")
    }

    pub fn view_proposals(&self, task_id: String) -> Vec<Proposal> {
        self.tasks_recores
            .get(&task_id)
            .expect("Not found this job in dupwork system")
            .proposals
            .to_vec()
    }

    pub fn task_by_id(&self, task_id: TaskId) -> WrappedTask {
        self.tasks_recores
            .get(&task_id)
            .map(|v| WrappedTask::from(v))
            .expect("Task not found")
    }
}
