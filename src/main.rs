mod lark;

use clap::Parser;
use lark::model::LarkSdk;
use lark::server::{index, not_found, send_text};
use rocket::{catchers, routes};
use std::{thread, time};

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
    let sdk = LarkSdk::new(args.app_id, args.app_secret).await;

    // let sdk = sdk::Sdk::new(args.app_id, args.app_secret).await;

    // let helper = helper::UserHelper::new(sdk);
    tokio::spawn(refresh_token(sdk.clone()));

    let figment = rocket::Config::figment()
        .merge(("address", args.address))
        .merge(("port", args.port));

    let _ = rocket::custom(figment)
        .manage(sdk)
        .mount("/", routes![index, send_text])
        .register("/", catchers![not_found])
        .launch()
        .await;
}

async fn refresh_token(sdk: LarkSdk) {
    let mut interval = 0;

    loop {
        let dur = time::Duration::from_secs(interval);
        thread::sleep(dur);

        println!("refresh_token... ");

        interval = match sdk.get_token().await {
            Ok(t) => {
                sdk.config
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
            sdk.config.get(&"token".to_string()),
            interval
        );
    }
}
