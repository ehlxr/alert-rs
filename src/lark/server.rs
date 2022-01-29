use std::error::Error;

use rocket::{
    catch, get, post,
    serde::json::{serde_json::json, Json, Value},
    State,
};

use tera::Context;
use tracing::{debug, info, warn};

use crate::{
    lark::model::{GroupTextMessage, LarkSdk, TextMessage},
    util::aes_cbc,
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
        context.insert("receive_id", &openid);

        let content = TEMPLATES.render("message.tmpl", &context).unwrap();
        if let Err(e) = sdk.message("open_id", content).await {
            status = format!("{}", e);
            break;
        }
    }

    json!({ "status": status })
}

async fn robot_echo(sdk: &LarkSdk, result: &Value) -> Result<(), Box<dyn Error>> {
    let message = &result["event"]["message"];

    let (mention_robot, mention_keys) = if let Some(mentions) = message["mentions"].as_array() {
        let mut is_robot = false;
        let mut mention_keys = vec![];
        for mention in mentions.iter() {
            mention_keys.push(mention["key"].as_str().unwrap_or_default());

            if mention["name"].as_str().unwrap_or_default() == sdk.robot_name {
                is_robot = true;
                break;
            }
        }
        (is_robot, mention_keys)
    } else {
        (false, vec![])
    };

    let chat_type = message["chat_type"].as_str().unwrap_or_default();
    if chat_type == "group" && !mention_robot {
        return Ok(());
    }

    let mut context = Context::new();
    context.insert(
        "receive_id",
        message["chat_id"].as_str().unwrap_or_default(),
    );

    let ct: Value =
        serde_json::from_str(message["content"].as_str().unwrap_or_default()).unwrap_or_default();
    let mut text = ct["text"].as_str().unwrap_or("hello").to_string();
    // 如果发送消息为： @用户1 @用户2 消息内容，
    // 接收到的 content text 内容为：@_user_1 @_user_2 消息内容， @_user_1 @_user_2 为 mention_key,
    for mention_key in mention_keys {
        text = text.replace(mention_key, "").trim().to_string();
    }
    context.insert("text", &text);

    if mention_robot {
        context.insert("at_id", &result["event"]["sender"]["sender_id"]["union_id"]);
    }

    sdk.message(
        "chat_id",
        TEMPLATES.render("message.tmpl", &context).unwrap(),
    )
    .await?;

    Ok(())
}

#[post("/event", format = "json", data = "<event>")]
pub async fn feishu_event(event: Json<Value>, sdk: &State<LarkSdk>) -> Value {
    let Json(value) = event;
    debug!("feishu event received param: {:?}", value);

    let decryptext = if let Some(encrypt_value) = value.get("encrypt") {
        match encrypt_value.as_str() {
            Some(encrypt) => {
                debug!("fetch encrypt from param: {}", encrypt);
                match aes_cbc::decrypt(&sdk.encrypt_key, encrypt) {
                    Ok(dt) => {
                        info!("decrypt result: {}", dt);
                        dt
                    }
                    Err(err) => return json!(format!("decrypt received param error {:?}", err)),
                }
            }
            None => return json!("encrypt string is none"),
        }
    } else {
        warn!("there is no encrypt field of received param!!!");
        value.to_string()
    };

    let result: Value = serde_json::from_str(&decryptext).unwrap();
    robot_echo(sdk, &result).await.unwrap();

    if let Some(challenge) = result["challenge"].as_str() {
        json!({ "challenge": challenge })
    } else {
        json!("ok")
    }
}
