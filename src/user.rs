use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    pub account_id: AccountId,
    pub bio: String,
    
    pub total_spent: Balance,
    pub total_earn: Balance,
    pub locked_balance: Balance,
    
    pub current_jobs: UnorderedSet<TaskId>,
    pub completed_jobs: UnorderedSet<TaskId>,
    
    pub pos_point: u32,
    pub neg_point: u32,
}

#[near_bindgen]
impl Dwork {
    pub fn user_info(&self, account_id: AccountId) -> Value {
        self.users
            .get(&account_id)
            .map(|v| {
                json!({
                    "account_id": v.account_id,
                    "bio": v.bio,
                    "completed_jobs": v.completed_jobs.to_vec(),
                    "current_jobs": v.current_jobs.to_vec(),
                    "total_earn": v.total_earn,
                    "total_spent": v.total_spent,
                    "locked_balance": v.locked_balance
                })
            })
            .expect("Canot map user to json")
    }
    
    #[payable]
    pub fn register(&mut self) {
        assert!(
            env::attached_deposit() == self.app_config.register_bond,
            "Send exactly {:?} Near to register",
            self.app_config.register_bond
        );

        let account_id = env::predecessor_account_id();
        let user = User {
            account_id: account_id.clone(),
            bio: "A member of dWork".to_string(),
            total_earn: 0,
            total_spent: 0,
            locked_balance: env::attached_deposit(),
            completed_jobs: UnorderedSet::new(StorageKey::UserCompletedTasks {
                account_id: account_id.clone(),
            }),
            current_jobs: UnorderedSet::new(StorageKey::UserCurrentTasks {
                account_id: account_id.clone(),
            }),
            pos_point: 0,
            neg_point: 0,
        };

        self.users.insert(&account_id, &user);
    }

    //Account logic
    pub fn update_bio(&mut self, bio: String) {
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User not found");

        user.bio = bio;
        self.users.insert(&account_id, &user);
    }

    #[private]
    pub fn add_pos_point(&mut self, account_id: AccountId, point: u32) {
        let mut user = self.users.get(&account_id).expect("User not found");
        let cur_point = user.pos_point;
        user.pos_point = cur_point + point;
        self.users.insert(&account_id, &user);
    }
    
    #[private]
    pub fn add_neg_point(&mut self, account_id: AccountId, point: u32) {
        let mut user = self.users.get(&account_id).expect("User not found");
        let cur_point = user.neg_point;
        user.neg_point = cur_point + point;
        self.users.insert(&account_id, &user);
    }
}
