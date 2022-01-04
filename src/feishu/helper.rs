use crate::{cache::Cache, feishu::sdk::Sdk};

pub struct UserHelper {
    sdk: Sdk,
    pub cache: Cache,
}

impl UserHelper {
    pub fn new(sdk: Sdk) -> Self {
        Self {
            sdk,
            cache: Cache::new(100, 6000),
        }
    }

    pub async fn get_ids(&self, mobiles: Vec<String>) -> Vec<String> {
        let mut no_id_mobiles = Vec::new();
        let mut ids = Vec::new();

        for mobile in mobiles {
            if let Some(cache_ids) = self.cache.get(&mobile) {
                ids.extend(cache_ids);
            } else {
                no_id_mobiles.push(mobile);
            }
        }

        if no_id_mobiles.len() <= 0 {
            return ids;
        }

        match self.sdk.batch_get_ids(no_id_mobiles).await {
            Ok(get_ids) => {
                for user in get_ids.data.user_list.into_iter() {
                    let uid = user.user_id.clone();
                    self.cache.insert(user.mobile, vec![user.user_id]).await;
                    ids.push(uid);
                }
            }
            Err(_) => todo!(),
        }

        return ids;
    }
}
