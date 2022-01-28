pub(crate) mod model;
pub(crate) mod server;

use self::model::*;
use moka::future::Cache;
use tracing::{error, info};

const GET_TOKEN_URL: &str = "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal";
const GET_ID_URL_V1: &str = "https://open.feishu.cn/open-apis/user/v1/batch_get_id?";
const GET_ID_URL_V3: &str = "https://open.feishu.cn/open-apis/contact/v3/users/batch_get_id";
const WEBHOOK_URL: &str = "https://open.feishu.cn/open-apis/bot/v2/hook/";
const MESSAGE_URL: &str = "https://open.feishu.cn/open-apis/im/v1/messages";

impl LarkConfig {
    fn new(max_capacity: usize) -> Self {
        Self {
            inner: Cache::new(max_capacity),
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn get(&self, key: &String) -> Option<String> {
        self.inner.get(key)
    }

    pub async fn insert(&self, key: String, value: String) {
        self.inner.insert(key, value).await;
    }
}

impl LarkSdk {
    pub async fn new(
        app_id: String,
        app_secret: String,
        cache_capacity: usize,
        bot_id: String,
        api_version: String,
        encrypt_key: String,
        robot_name: String,
    ) -> Self {
        Self {
            app_id,
            app_secret,
            bot_id,
            config: LarkConfig::new(cache_capacity),
            api_version,
            encrypt_key,
            robot_name,
        }
    }

    pub async fn get_token(&self) -> Result<TokenResponse, reqwest::Error> {
        let new_post = TokenRequest {
            app_id: &self.app_id,
            app_secret: &self.app_secret,
        };

        // let res: TokenResponse = reqwest::blocking::Client::new()
        //     .post(GET_TOKEN_URL)
        //     .json(&new_post)
        //     .send()?
        //     .json()?;

        let res: TokenResponse = reqwest::Client::new()
            .post(GET_TOKEN_URL)
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    async fn batch_get_ids_v3(
        &self,
        mobiles: Vec<String>,
    ) -> Result<GetIDResponse<GetIDResponseDataV3>, reqwest::Error> {
        let new_post = GetIDRequest { mobiles };
        // let res: GetIDResponse = reqwest::blocking::Client::new()
        //     .post(GET_ID_URL_V3)
        //     .header("Authorization", format!("Bearer {}", self.token))
        //     .json(&new_post)
        //     .send()?
        //     .json()?;
        // Ok(res)

        let res = reqwest::Client::new()
            .post(GET_ID_URL_V3)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.config
                        .get(&"token".to_string())
                        .expect("token is none")
                ),
            )
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;

        info!("{:?}", res);
        Ok(res)
    }

    async fn batch_get_ids(
        &self,
        mobiles: Vec<String>,
    ) -> Result<GetIDResponse<GetIDResponseData>, reqwest::Error> {
        let mut api = GET_ID_URL_V1.to_string();
        for mobile in mobiles {
            api = format!("{}mobiles={}&", api, mobile);
        }
        let api = &api[0..api.len() - 1];

        let res = reqwest::Client::new()
            .get(api)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.config
                        .get(&"token".to_string())
                        .expect("token is none")
                ),
            )
            .send()
            .await?
            .json()
            .await?;

        info!("{:?}", res);

        Ok(res)
    }

    pub async fn get_ids(&self, mobiles: Vec<String>) -> Vec<String> {
        let mut no_id_mobiles = Vec::new();
        let mut ids = Vec::new();

        for mobile in mobiles {
            if let Some(cache_id) = self.config.get(&mobile) {
                // ids.extend(cache_id);
                ids.push(cache_id);
            } else {
                no_id_mobiles.push(mobile);
            }
        }

        if no_id_mobiles.len() <= 0 {
            return ids;
        }

        if "v1" == self.api_version {
            match self.batch_get_ids(no_id_mobiles).await {
                Ok(get_ids) => {
                    for (mobile, user) in get_ids.data.mobile_users.into_iter() {
                        let open_id = &user.get(0).unwrap().open_id;
                        self.config.insert(mobile, open_id.clone()).await;
                        ids.push(open_id.to_string());
                    }
                }
                Err(err) => error!("get user id error {}", err),
            }
        } else {
            match self.batch_get_ids_v3(no_id_mobiles).await {
                Ok(get_ids) => {
                    for user in get_ids.data.user_list.into_iter() {
                        let uid = user.user_id.clone();
                        self.config.insert(user.mobile, user.user_id).await;
                        ids.push(uid);
                    }
                }

                Err(err) => error!("get user id error {}", err),
            }
        }

        return ids;
    }

    pub async fn webhook(
        &self,
        bot_id: String,
        content: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut bid = &bot_id;
        if bot_id == "" {
            bid = &self.bot_id;
        }
        let api = format!("{}{}", WEBHOOK_URL, bid);

        let res = reqwest::Client::new()
            .post(api)
            .body(content)
            .send()
            .await?;

        info!("{:?}", res);

        Ok(res)
    }

    pub async fn message(
        &self,
        receive_id_type: &str,
        content: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let res = reqwest::Client::new()
            .post(format!(
                "{}?receive_id_type={}",
                MESSAGE_URL, receive_id_type
            ))
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.config
                        .get(&"token".to_string())
                        .expect("token is none")
                ),
            )
            .body(content)
            .send()
            .await?;

        info!("{:?}", res);
        Ok(res)
    }
}
