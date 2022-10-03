use std::ops::Range;

use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    StorageAccount,
    TaskRecores,
    Proposals,
    Reports,
    Users,
    Categories,
    UserLockedBalance {account_id: AccountId},
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

pub fn calculate_rev_limit(len: u64, from_index: u64, limit: u64) -> Range<u64> {
    let from = if len > (limit + from_index) {
        len - limit - from_index
    } else {
        0
    };

    let to = if len > from_index {
        len - from_index
    } else {
        0
    };

    from..to
}
