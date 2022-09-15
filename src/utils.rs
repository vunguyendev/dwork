use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    TaskRecores,
    Users,
    Categories,
    UserCurrentTasks { account_id: AccountId },
    UserCompletedTasks { account_id: AccountId },
    ProposalsPerTask { task_id: String },
    Admins,
}
