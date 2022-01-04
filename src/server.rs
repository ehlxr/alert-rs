use rocket::{
    catch, get, post,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
    State,
};

use crate::feishu::helper::{self, UserHelper};

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
pub fn index() -> String {
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
pub fn send_text(message: Json<TextMessage>, helper: &State<UserHelper>) -> Value {
    let Json(msg) = message;
    println!("{:?}", msg);

    match helper.cache.get(&msg.at) {
        Some(ids) => match ids.get(0) {
            Some(id) => {
                println!("{}", id)
            }
            None => todo!(),
        },
        None => todo!(),
    };

    json!({ "status": "ok"})
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TextMessage {
    at: String,
    text: String,
    bot_id: String,
    // open_ids: String,
}
