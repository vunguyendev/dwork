use super::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Category {
    id: CategoryId,
    name: String,
    created: u64,
    pub num_posts: u64,
}

pub type CategoryId = String;

#[near_bindgen]
impl Dwork {
    pub fn categories(&self, from_index: u64, limit: u64) -> Vec<Category> {
        let category_ids = self.categories.keys_as_vector();

        calculate_rev_limit(category_ids.len(), from_index, limit)
            .map(|index| {
                let category_id = category_ids.get(index).unwrap();
                self.categories.get(&category_id).unwrap()
            })
            .rev()
            .collect()
    }

    pub fn new_category(&mut self, topic_name: String) -> bool {
        let topic_id = topic_name.to_lowercase().replace(' ', "_");

        assert!(
            topic_name.len() <= self.app_config.maximum_title_length.into(),
            "Can not make a post title more than {} characters",
            self.app_config.maximum_title_length
        );

        assert!(
            self.categories.get(&topic_id).is_none(),
            "Topic already exists"
        );

        // let account_id = env::predecessor_account_id();
        // let storage_update = self.new_storage_update(account_id.clone());

        let topic = Category {
            id: topic_id.clone(),
            name: topic_name,
            created: env::block_timestamp(),
            num_posts: 0,
        };

        self.categories.insert(&topic_id, &topic);
        // self.finalize_storage_update(storage_update);
        true
    }
}
