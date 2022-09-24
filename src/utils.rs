use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    StorageAccount,
    TaskRecores,
    Proposals,
    Reports,
    Users,
    Categories,
    UserCurrentTasks { account_id: AccountId },
    UserCompletedTasks { account_id: AccountId },
    // ProposalsPerTask { task_id: String },
    Admins,
}

pub fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

