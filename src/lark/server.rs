use rocket::{
    catch, get, post,
    serde::json::{serde_json::json, Json, Value},
    State,
};
use std::str;
use tera::Context;
use tracing::{error, info};

use crate::{
    lark::model::{GroupTextMessage, LarkSdk, TextMessage},
    util::{self, aes_cbc},
    CACHE, TEMPLATES,
};

// #[rocket::main]
// pub async fn launch_rocket() -> Result<(), rocket::Error> {
//     rocket::build()
//         // .mount("/", routes![index])
//         .mount("/", routes![index])
//         .register("/", catchers![not_found])
//         .launch()
//         .await
// }

#[get("/")]
pub async fn index(sdk: &State<LarkSdk>) -> String {
    info!(
        "config cache token {}",
        sdk.config.get(&"token".to_string()).unwrap()
    );
    info!(
        "cache token {}",
        CACHE.read().unwrap().get(&"token".to_string()).unwrap()
    );
    "hello".to_string()
}

#[catch(404)]
pub fn not_found() -> Result<Value, ()> {
    Ok(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[post("/group/message", format = "json", data = "<message>")]
pub async fn group_message(message: Json<GroupTextMessage>, sdk: &State<LarkSdk>) -> Value {
    let Json(msg) = message;
    info!("receive message: {:?}", msg);

    let ids = sdk
        .get_ids(msg.mobiles.split(",").map(|x| x.to_string()).collect())
        .await;

    let mut context = Context::new();
    context.insert("text", &msg.text);
    context.insert("openids", &ids);
    let content = TEMPLATES.render("group_message.tmpl", &context).unwrap();

    let mut status = String::from("ok");
    if let Err(e) = sdk.webhook(msg.bot_id, content).await {
        status = format!("{}", e)
    }

    json!({ "status": status })
}

#[post("/message", format = "json", data = "<message>")]
pub async fn message(message: Json<TextMessage>, sdk: &State<LarkSdk>) -> Value {
    let Json(msg) = message;

    let ids = sdk
        .get_ids(msg.mobiles.split(",").map(|x| x.to_string()).collect())
        .await;

    let mut context = Context::new();
    let mut status = String::from("ok");

    for openid in ids {
        context.insert("text", &msg.text);
        context.insert("openid", &openid);
        let content = TEMPLATES.render("message.tmpl", &context).unwrap();

        if let Err(e) = sdk.message(content).await {
            status = format!("{}", e);
            break;
        }
    }

    json!({ "status": status })
}

#[post("/event", format = "json", data = "<event>")]
pub async fn feishu_event(event: Json<Value>) -> Value {
    let Json(value) = event;
    info!("received value: {:?}", value);

    if let Some(encrypt_value) = value.get("encrypt") {
        if let Some(encrypt) = encrypt_value.as_str() {
            info!("received encrypt: {}", encrypt);
            // let decode_str = &match base64::decode(encrypt) {
            //     Ok(d) => d,

            //     Err(e) => {
            //         panic!("decode {}", e);
            //     }
            // };
            let dec = util::decrtypt("vaq9ohuOdiYf8Q9UlxSz6bF5ZQqjPmpO", encrypt);
            info!("des: {}", dec);

            // match str::from_utf8(decode_str) {
            //     Ok(r) => {
            //         info!("{}", r);
            //         let dec = aes_cbc::decrypt("key", r);
            //         info!("des: {}", dec);
            //     }
            //     Err(e) => {
            //         error!("fromutf8 {}", e);
            //     }
            // }
        }
    }

    json!({ "challenge": "ok" })
}
