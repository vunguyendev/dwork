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
    pub fn get_reports(&self, from_index: u64, limit: u64) -> Vec<Report> {
        // let caller = env::predecessor_account_id();
        // assert!(self.is_admin(caller), "Just admin can call this function");
        let reports = self.reports.keys_as_vector();
        
        calculate_rev_limit(reports.len(), from_index, limit)
            .map(|index| {
                let key = reports.get(index).unwrap();
                self.reports.get(&key).unwrap()
            })
            .rev()
            .collect()
    }

    pub fn get_report_by_id(&self, report_id: ReportId) -> Report {
        self.reports.get(&report_id).expect("Report not found")
    }
}
