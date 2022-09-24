use crate::*;

pub type TaskId = String;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskStatus {
    SubmitAndReview,
    ReportAndReviewReport,
    Complete,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Task {
    pub owner: AccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub price: Balance,
    pub proposals: Vec<ProposalId>,
    pub approved: Vec<ProposalId>, // Proposal id is account_id of worker
    pub created_at: Timestamp,

    // As default: after deadline (submit_available_until), owner have 1 month to review all submition and marks task as
    // complete if done soon. from that time (review_proposal_complete_at) all worker have 2 days more to review and submit the
    // report
    pub submit_available_until: Timestamp,
    pub review_proposal_complete_at: Option<Timestamp>,

    pub category_id: CategoryId,
}

#[near_bindgen]
impl Dwork {
    pub(crate) fn internal_get_task(&self, task_id: &TaskId) -> Task {
        self.task_recores.get(&task_id).expect("Task not found")
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
}
