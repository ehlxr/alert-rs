mod lark;

use clap::Parser;
use lark::model::LarkSdk;
use lark::server::{index, not_found, send_text};
use rocket::{catchers, routes};

use std::sync::Mutex;
use std::{thread, time};
use tera::Tera;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".tmpl"]);
        tera
    };
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// BotId feishu webhook group bot id addr
    #[clap(short, long, default_value = "d66fffcc-c6af-406d-a4c3-96cb112f9fca")]
    bot_id: String,

    /// IP address to serve on
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Port to serve on
    #[clap(short, long, default_value_t = 8000)]
    port: u16,

    /// Cache Capacity max capacity of config cache
    #[clap(short, long, default_value_t = 100)]
    cache_capacity: usize,

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

    let sdk = LarkSdk::new(
        args.app_id,
        args.app_secret,
        args.cache_capacity,
        args.bot_id,
    )
    .await;

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

        interval = if let Ok(t) = sdk.get_token().await {
            sdk.config
                .insert("token".to_string(), t.tenant_access_token)
                .await;
            // https://open.feishu.cn/document/ukTMukTMukTM/uIjNz4iM2MjLyYzM
            // Token 有效期为 2 小时，在此期间调用该接口 token 不会改变。
            // 当 token 有效期小于 30 分的时候，再次请求获取 token 的时候，会生成一个新的 token，与此同时老的 token 依然有效。
            // 在过期前 1 分钟刷新
            (t.expire - 60).try_into().unwrap()
        } else {
            0
        };

        println!(
            "current token is {} will refresh after {}s",
            sdk.config.get(&"token".to_string()).unwrap(),
            interval
        );
    }
}
