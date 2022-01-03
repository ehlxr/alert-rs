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
    pub fn new(app_id: String, app_secret: String) -> Self {
        let mut sdk = Self {
            app_id,
            app_secret,
            token: "".to_string(),
        };

        match sdk.get_token() {
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

    pub fn get_token(&self) -> Result<TokenResponse, reqwest::Error> {
        let new_post = TokenRequest {
            app_id: &self.app_id,
            app_secret: &self.app_secret,
        };

        let res: TokenResponse = reqwest::blocking::Client::new()
            .post("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
            .json(&new_post)
            // .body("the exact body that is sent")
            .send()?
            .json()?;

        println!("{:#?}", res);

        Ok(res)
    }
}
