use clap::Parser;

use feishu::helper::{self, UserHelper};
use rocket::{catchers, routes};
use server::{index, not_found, send_text};
use tokio::time;

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
    let h = helper.clone();
    tokio::spawn(refreshToken(h));

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

async fn refreshToken(helper: UserHelper) {
    match helper.sdk.get_token().await {
        Ok(t) => {
            helper
                .cache
                .insert("token".to_string(), t.tenant_access_token)
                .await;
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    let mut interval = time::interval(time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        println!("2333");
    }
}
