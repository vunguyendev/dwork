use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LockedBalance {
    // pub task_id: TaskId,
    pub amount: Balance,
    pub release_at: Timestamp,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub account_id: AccountId,
    pub bio: String,

    pub total_spent: Balance,
    pub total_earn: Balance,
    pub balance: Balance,

    pub locked_balance: UnorderedMap<TaskId, LockedBalance>,

    pub current_jobs: UnorderedSet<TaskId>,
    pub completed_jobs: UnorderedSet<TaskId>,

    pub pos_point: u32,
    pub neg_point: u32,
}

#[near_bindgen]
impl Dwork {
    pub fn user_info(&self, account_id: AccountId) -> Value {
        self.accounts
            .get(&account_id)
            .map(|v| {
                json!({
                    "account_id": v.account_id,
                    "bio": v.bio,
                    "completed_jobs": v.completed_jobs.to_vec(),
                    "current_jobs": v.current_jobs.to_vec(),
                    "total_earn": v.total_earn,
                    "total_spent": v.total_spent,
                    "balance": v.balance
                })
            })
            .expect("Canot map user to json")
    }

    pub(crate) fn internal_create_account(&mut self, account_id: &AccountId) -> Account {
        let account = Account {
            account_id: account_id.clone(),
            bio: "A member of dWork".to_string(),
            total_earn: 0,
            total_spent: 0,
            balance: env::attached_deposit(),
            locked_balance: UnorderedMap::new(StorageKey::UserLockedBalance {
                account_id: account_id.clone(),
            }),
            completed_jobs: UnorderedSet::new(StorageKey::UserCompletedTasks {
                account_id: account_id.clone(),
            }),
            current_jobs: UnorderedSet::new(StorageKey::UserCurrentTasks {
                account_id: account_id.clone(),
            }),
            pos_point: 0,
            neg_point: 0,
        };

        assert!(
            self.accounts.get(account_id).is_none(),
            "Account already exists"
        );

        self.accounts.insert(account_id, &account);
        account
    }

    pub(crate) fn internal_get_account_optional(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id)
    }

    pub(crate) fn internal_get_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account_optional(account_id)
            .expect("Account doesn't exist")
    }

    pub(crate) fn internal_set_account(&mut self, account_id: &AccountId, account: Account) {
        self.accounts.insert(account_id, &account);
    }

    pub fn update_bio(&mut self, bio: String) {
        let account_id = env::predecessor_account_id();
        let mut account = self.internal_get_account(&account_id);
        account.bio = bio;
        self.internal_set_account(&account_id, account);
    }

    #[private]
    pub fn add_pos_point(&mut self, account_id: AccountId, point: u32) {
        let mut account = self.internal_get_account(&account_id);
        let cur_point = account.pos_point;
        account.pos_point = cur_point + point;
        self.internal_set_account(&account_id, account);
    }

    #[private]
    pub fn add_neg_point(&mut self, account_id: AccountId, point: u32) {
        let mut account = self.internal_get_account(&account_id);
        let cur_point = account.neg_point;
        account.neg_point = cur_point + point;
        self.internal_set_account(&account_id, account);
    }
}
