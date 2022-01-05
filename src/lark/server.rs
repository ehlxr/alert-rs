use rocket::{
    catch, get, post,
    serde::json::{serde_json::json, Json, Value},
    State,
};
use tera::Context;

use crate::{
    lark::model::{LarkSdk, TextMessage},
    ARRAY, TEMPLATES,
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

    let mut context = Context::new();
    context.insert("text", &msg.text);
    context.insert("openids", &ids);
    let content = TEMPLATES.render("text.tmpl", &context).unwrap();

    let mut status = String::from("ok");
    if let Err(e) = sdk.webhook(msg.bot_id, content).await {
        status = format!("{}", e)
    }

    ARRAY.lock().unwrap().push(1);
    println!("called {}", ARRAY.lock().unwrap().len());

    json!({ "status": status })
}
