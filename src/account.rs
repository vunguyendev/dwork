use crate::*;
use near_sdk::json_types::U128;
use std::convert::TryInto;

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LockedBalance {
    pub amount: Balance,
    pub release_at: Timestamp,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub account_id: AccountId,
    pub bio: String,

    pub total_spent: Balance,
    pub total_earn: Balance,

    pub locked_balance: UnorderedMap<TaskId, LockedBalance>,

    pub current_jobs: UnorderedSet<TaskId>,
    pub completed_jobs: UnorderedSet<TaskId>,

    pub pos_point: u32,
    pub neg_point: u32,
}

impl Account {
    pub fn add_pos_point(&mut self, point: u32) {
        self.pos_point += point
    }
    
    pub fn add_neg_point(&mut self, point: u32) {
        self.neg_point += point
    }
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedAccount {
    pub account_id: String,
    pub bio: String,
    pub locked_balance: Vec<(TaskId, LockedBalance)>,
    pub balance: Option<AccountStorageBalance>,

    pub total_spent: U128,
    pub total_earn: U128,

    pub current_jobs: Vec<TaskId>,
    pub completed_jobs: Vec<TaskId>,

    pub pos_point: u32,
    pub neg_point: u32,
}

impl From<Account> for WrappedAccount {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.account_id,
            bio: account.bio,
            locked_balance: account.locked_balance.iter().map(|(k, v)| (k, v)).collect(),
            balance: None,

            total_spent: account.total_spent.into(),
            total_earn: account.total_earn.into(),

            current_jobs: account.current_jobs.to_vec(),
            completed_jobs: account.completed_jobs.to_vec(),

            pos_point: account.pos_point,
            neg_point: account.neg_point,
        }
    }
}

#[near_bindgen]
impl Dwork {
    pub fn user_info(&self, account_id: AccountId) -> WrappedAccount {
        let mut wrapped_account: WrappedAccount = self
            .accounts
            .get(&account_id)
            .expect("Account not found")
            .into();
        let storage_balance = self.storage_balance_of((account_id.as_str()).try_into().unwrap());
        wrapped_account.balance = Some(storage_balance);
        wrapped_account
    }

    // Modify method
    pub fn update_bio(&mut self, bio: String) {
        let account_id = env::predecessor_account_id();
        let mut account = self.internal_get_account(&account_id);
        account.bio = bio;
        self.internal_set_account(&account_id, account);
    }

    // #[payable]
    // pub fn deposit(&mut self, account_id: Option<AccountId>) {
    //     let account_id = account_id.unwrap_or_else(env::predecessor_account_id);
    //     let mut account = self.internal_get_account(&account_id);
    //
    //     assert!(
    //         env::attached_deposit() >= self.app_config.minimum_deposit
    //             && env::attached_deposit() <= self.app_config.maximum_deposit,
    //         "Total amount for each task must be in a range from {} to {}",
    //         self.app_config.minimum_deposit,
    //         self.app_config.maximum_deposit
    //     );
    //
    //     account.balance += env::attached_deposit();
    //     self.internal_set_account(&account_id, account)
    // }
    //
    // pub fn withdraw(&mut self, account_id: Option<AccountId>, amount: Balance) {
    //     let account_id = account_id.unwrap_or_else(env::predecessor_account_id);
    //     let account = self.internal_get_account(&account_id);
    //
    //     assert!(
    //         account.balance >= amount,
    //         "Account doesn't have enough balance"
    //     );
    //
    //     assert!(
    //         account.pos_point > 50,
    //         "Account must have positive point higher than {}",
    //         50
    //     );
    //
    //     if account.neg_point != 0 {
    //         let rate = account.pos_point / account.neg_point;
    //         assert!(
    //             rate > self.app_config.critical_point as u32,
    //             "Account must have positive point / negative point higher than {}",
    //             self.app_config.critical_point
    //         );
    //     }
    //
    //     assert!(
    //         amount >= self.app_config.minimum_deposit && amount <= self.app_config.maximum_deposit,
    //         "Amount for each withdraws must be in a range from {} to {}",
    //         self.app_config.minimum_deposit,
    //         self.app_config.maximum_deposit
    //     );
    //
    //     Promise::new(account_id.to_string())
    //         .transfer(amount)
    //         .then(ext_self::on_transferd(
    //             account_id,
    //             amount,
    //             &env::current_account_id(),
    //             0,
    //             DEFAULT_GAS_TO_PAY,
    //         ));
    // }

    pub(crate) fn internal_create_account(&mut self, account_id: &AccountId) -> Account {
        let account = Account {
            account_id: account_id.clone(),
            bio: "A member of dWork".to_string(),
            total_earn: 0,
            total_spent: 0,
            // balance: env::attached_deposit(),
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
}
