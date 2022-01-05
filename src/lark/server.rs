use rocket::{
    catch, get, post,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
    State,
};

use crate::lark::model::LarkSdk;

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
    println!("cache token {:?}", sdk.config.get(&"token".to_string()));

    "hello".to_string()
}

#[catch(404)]
pub fn not_found() -> Result<Value, ()> {
    Ok(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[post("/sendText", format = "json", data = "<message>")]
pub async fn send_text(message: Json<TextMessage>, sdk: &State<LarkSdk>) -> Value {
    let Json(msg) = message;

    let ids = sdk
        .get_ids(msg.at.split(",").map(|x| x.to_string()).collect())
        .await;

    json!({ "status": "ok","ids": ids})
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TextMessage {
    at: String,
    text: String,
    bot_id: String,
    // open_ids: String,
}
