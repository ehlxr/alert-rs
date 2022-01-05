use clap::Parser;

use feishu::helper::{self, UserHelper};
use rocket::{catchers, routes};
use server::{index, not_found, send_text};
use std::{thread, time};

use crate::feishu::sdk;

mod cache;
mod feishu;
mod server;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// BotId feishu webhook group bot id addr
    #[clap(
        short,
        long,
        default_value = "hook/d66fffcc-c6af-406d-a4c3-96cb112f9fca"
    )]
    bot_id: String,

    /// IP address to serve on
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Port to serve on
    #[clap(short, long, default_value_t = 8000)]
    port: u16,

    /// AppID id of feishu app for get user open id
    #[clap(short = 'i', long, default_value = "")]
    app_id: String,

    /// AppSecret secret of feishu app for get user open id
    #[clap(short = 's', long, default_value = "")]
    app_secret: String,

    /// Verbose show verbose log"
    #[clap(short, long)]
    verbose: bool,
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    let sdk = sdk::Sdk::new(args.app_id, args.app_secret).await;

    let helper = helper::UserHelper::new(sdk);
    tokio::spawn(refresh_token(helper.clone()));

    let figment = rocket::Config::figment()
        .merge(("address", args.address))
        .merge(("port", args.port));

    let _ = rocket::custom(figment)
        .manage(helper)
        .mount("/", routes![index, send_text])
        .register("/", catchers![not_found])
        .launch()
        .await;
}

async fn refresh_token(helper: UserHelper) {
    let mut interval = 0;

    loop {
        let dur = time::Duration::from_secs(interval);
        thread::sleep(dur);

        println!("refresh_token... ");

        interval = match helper.sdk.get_token().await {
            Ok(t) => {
                helper
                    .cache
                    .insert("token".to_string(), t.tenant_access_token)
                    .await;
                (t.expire - 600).try_into().unwrap()
            }
            Err(e) => {
                println!("{}", e);
                0
            }
        };
        println!(
            "current token is {:?} will refresh after {:?}",
            helper.cache.get(&"token".to_string()),
            interval
        );
    }
}
