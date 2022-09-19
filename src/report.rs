use crate::*;

pub type ReportId = String;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum ReportStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Report {
    pub report_id: ReportId,
    pub account_id: AccountId,
    pub task_id: TaskId,
    pub report: String, //prefer an url like github repo or figma design files, etc
    pub status: ReportStatus,
}

#[near_bindgen]
impl Dwork {
    pub fn get_all_reports(&self) -> Vec<Report> {
        let caller = env::predecessor_account_id();
        assert!(self.is_admin(caller), "Just admin can call this function");

        self.reports
            .values_as_vector()
            .iter()
            .collect()
    }   
}
