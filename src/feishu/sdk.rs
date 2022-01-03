use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct Sdk {
    app_id: String,
    app_secret: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenRequest<'a> {
    app_id: &'a str,
    app_secret: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    code: i32,
    msg: String,
    tenant_access_token: String,
    expire: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetIDResponseDataUser {
    mobile: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetIDRequest {
    mobiles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetIDResponseData {
    user_list: Vec<GetIDResponseDataUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIDResponse {
    code: i32,
    msg: String,
    data: GetIDResponseData,
}

// pub async fn get_token(sdk: Sdk) -> Result<(), reqwest::Error> {
//     let new_post = TokenRequest {
//         app_id: &sdk.app_id,
//         app_secret: &sdk.app_secret,
//     };
//     let new_post: TokenResponse = reqwest::Client::new()
//         .post("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
//         .json(&new_post)
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("{:#?}", new_post);

//     Ok(())
// }

impl Sdk {
    pub async fn new(app_id: String, app_secret: String) -> Self {
        let mut sdk = Self {
            app_id,
            app_secret,
            token: "".to_string(),
        };

        match sdk.get_token().await {
            Ok(t) => {
                sdk.token = t.tenant_access_token;
            }
            Err(e) => {
                // todo!()
                println!("{}", e)
            }
        }
        sdk
    }

    async fn get_token(&self) -> Result<TokenResponse, reqwest::Error> {
        let new_post = TokenRequest {
            app_id: &self.app_id,
            app_secret: &self.app_secret,
        };

        // let res: TokenResponse = reqwest::blocking::Client::new()
        //     .post("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
        //     .json(&new_post)
        //     .send()?
        //     .json()?;

        let res: TokenResponse = reqwest::Client::new()
            .post("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", res);

        Ok(res)
    }

    pub fn batch_get_ids(&self, mobiles: Vec<String>) -> Result<GetIDResponse, reqwest::Error> {
        // let mut api = "https://open.feishu.cn/open-apis/user/v1/batch_get_id?".to_string();
        // for mobile in mobiles {
        //     api = format!("{}mobiles={}&", api, mobile);
        // }
        // let api = &api[0..api.len()];

        let new_post = GetIDRequest { mobiles };

        let res: GetIDResponse = reqwest::blocking::Client::new()
            .post("https://open.feishu.cn/open-apis/contact/v3/users/batch_get_id")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&new_post)
            .send()?
            .json()?;

        println!("{:#?}", res);

        Ok(res)
    }
}
