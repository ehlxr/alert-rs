use crate::{cache::Cache, feishu::sdk::Sdk};

pub struct UserHelper {
    sdk: Sdk,
    pub cache: Cache,
}

impl UserHelper {
    pub fn new(sdk: Sdk) -> Self {
        Self {
            sdk,
            cache: Cache::new(100),
        }
    }

    pub async fn get_ids(&self, mobiles: Vec<String>) -> Vec<String> {
        let mut no_id_mobiles = Vec::new();
        let mut ids = Vec::new();

        for mobile in mobiles {
            if let Some(cache_id) = self.cache.get(&mobile) {
                // ids.extend(cache_id);
                ids.push(cache_id);
            } else {
                no_id_mobiles.push(mobile);
            }
        }

        if no_id_mobiles.len() <= 0 {
            return ids;
        }

        match self.sdk.batch_get_ids(no_id_mobiles).await {
            Ok(get_ids) => {
                // for user in get_ids.data.user_list.into_iter() {
                //     let uid = user.user_id.clone();
                //     self.cache.insert(user.mobile, vec![user.user_id]).await;
                //     ids.push(uid);
                // }
                for (mobile, user) in get_ids.data.mobile_users.into_iter() {
                    let open_id = &user.get(0).unwrap().open_id;
                    self.cache.insert(mobile, open_id.clone()).await;
                    ids.push(open_id.to_string());
                }
            }
            Err(err) => println!("{}", err),
        }

        return ids;
    }
}
