use std::collections::HashMap;

use moka::future::Cache;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct LarkSdk {
    // `pub(crate)` 使得只在当前 crate 中可见
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    pub(crate) bot_id: String,
    pub config: LarkConfig,
    pub(crate) api_version: String,
    pub(crate) encrypt_key: String,
}

#[derive(Clone)]
pub struct LarkConfig {
    pub(crate) inner: Cache<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest<'a> {
    pub app_id: &'a str,
    pub app_secret: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    code: i32,
    msg: String,
    pub tenant_access_token: String,
    pub expire: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDRequest {
    pub(crate) mobiles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponseDataV3 {
    #[serde(default)]
    pub user_list: Vec<GetIDResponseDataUserV3>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponseDataUserV3 {
    pub mobile: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponseData {
    #[serde(default)] // 如果反序列化时不存在该值，则使用 Default::default()
    pub mobile_users: HashMap<String, Vec<GetIDResponseDataUser>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponseDataUser {
    pub user_id: String,
    pub open_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponse<T> {
    code: i32,
    msg: String,
    pub data: T,
    // pub data: GetIDResponseData,
    // pub data: GetIDResponseDataV3,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GroupTextMessage {
    #[serde(default)] // 如果反序列化时不存在该值，则使用 Default::default()
    pub(crate) mobiles: String,
    pub(crate) text: String,
    #[serde(default)]
    pub(crate) bot_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TextMessage {
    pub(crate) mobiles: String,
    pub(crate) text: String,
}
