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

    pub async fn get_open_ids(&self, mobiles: Vec<String>) {
        let mut noIdMobiles = Vec::new();
        for mobile in mobiles {
            self.cache.get(&mobile).unwrap_or_else(|mobile| {
                noIdMobiles.push(mobile);
            });
        }

        if noIdMobiles.len() <= 0 {
            return;
        }

        match self.sdk.batch_get_ids(noIdMobiles).await {
            Ok(ids) => {
                for id in ids.data.user_list.into_iter() {
                    self.cache.insert(id.mobile, vec![id.user_id]).await;
                }
            }
            Err(_) => todo!(),
        }
    }
}
