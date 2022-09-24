use crate::*;

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
        let proposals: Vec<Proposal> = task
            .proposals
            .iter()
            .map(|item| self.proposals.get(item).expect("Proposal not found"))
            .collect();

        WrappedTask {
            owner: task.owner,
            title: task.title,
            description: task.description,
            max_participants: task.max_participants,
            price: WrappedBalance::from(task.price),
            proposals,
            created_at: WrappedTimestamp::from(task.created_at),
            available_until: WrappedTimestamp::from(task.submit_available_until),
            category_id: task.category_id,
        }
    }
}

// #[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct WrappedUser {
//     pub account_id: ValidAccountId,
//     pub bio: String,
//     pub user_type: UserType,
//     pub completed_jobs: Vec<TaskId>,
// }
