use crate::*;

pub type ProposalId = String;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected {
        reason: String,
        reject_at: Timestamp,
        report_id: Option<ReportId>,
    },
    ApprovedByAdmin {account_id: AccountId},
    RejectedByAdmin {account_id: AccountId}
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub account_id: AccountId,
    pub submit_time: Timestamp,
    pub proof_of_work: String, //prefer an url like github repo or figma design files, etc
    pub status: ProposalStatus,
}
