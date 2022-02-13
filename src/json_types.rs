use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTask {
    pub owner: ValidAccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub hour_rate: WrappedBalance,
    pub hour_estimation: Duration,
    pub proposals: Vec<WrappedProposal>,
    pub status: JobStatus,
}

impl From<Task> for WrappedTask {
    fn from(task: Task) -> Self {
        let wrapped_proposal: Vec<WrappedProposal> = task
            .proposals
            .iter()
            .map(|item| WrappedProposal::from(item))
            .collect();

        WrappedTask {
            owner: task.owner,
            title: task.title,
            description: task.description,
            max_participants: task.max_participants,
            hour_rate: WrappedBalance::from(task.hour_rate),
            hour_estimation: task.hour_estimation,
            proposals: wrapped_proposal,
            status: task.status,
        }
    }
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedUser {
    pub account_id: ValidAccountId,
    pub user_type: UserType,
    pub completed_jobs: Vec<TaskId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum WrappedUserType {
    Requester {
        total_transfered: WrappedBalance,
        current_requests: u16,
    },
    Worker {
        total_received: WrappedBalance,
        current_applies: u16,
    },
}

impl From<UserType> for WrappedUserType {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::Requester {
                total_transfered,
                current_requests,
            } => WrappedUserType::Requester {
                total_transfered: WrappedBalance::from(total_transfered),
                current_requests,
            },
            UserType::Worker {
                total_received,
                current_applies,
            } => WrappedUserType::Worker {
                total_received: WrappedBalance::from(total_received),
                current_applies,
            },
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedProposal {
    pub account_id: ValidAccountId,
    pub cover_letter: String,
    pub hour_estimation: Duration,
    pub total_received: WrappedBalance,
    pub proof_of_work: String, //prefer an url like github repo or figma design files, etc
}

impl From<Proposal> for WrappedProposal {
    fn from(proposal: Proposal) -> Self {
        WrappedProposal {
            account_id: proposal.account_id,
            cover_letter: proposal.cover_letter,
            hour_estimation: proposal.hour_estimation,
            total_received: WrappedBalance::from(proposal.total_received),
            proof_of_work: proposal.proof_of_work,
        }
    }
}
