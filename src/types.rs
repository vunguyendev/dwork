use crate::*;

// pub type TaskId = String;
//
// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct Task {
//     pub owner: AccountId,
//     pub title: String,
//     pub description: String,
//     pub max_participants: u16,
//     pub price: Balance,
//     pub proposals: UnorderedMap<AccountId, Proposal>,
//     pub approved: Vec<String>,
//     pub created_at: Timestamp,
//     pub available_until: Timestamp,
//     pub category_id: CategoryId,
// }

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct User {
//     pub account_id: AccountId,
//     pub bio: String,
//     pub total_spent: Balance,
//     pub total_earn: Balance,
//     pub locked_balance: Balance,
//     pub current_jobs: UnorderedSet<TaskId>,
//     pub completed_jobs: UnorderedSet<TaskId>,
// }

pub type ProposalId = String;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected {reason: String},
    Reported {report_id: ReportId}
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub account_id: AccountId,
    pub submit_time: Timestamp,
    pub proof_of_work: String, //prefer an url like github repo or figma design files, etc
    pub status: ProposalStatus,
}


