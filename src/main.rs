mod lark;
mod time_test;

use clap::Parser;
use lark::model::LarkSdk;
use lark::server::{group_message, index, message, not_found};
use rocket::{catchers, routes};
use time::format_description;
use time::macros::offset;
use tracing_subscriber::fmt::time::OffsetTime;

use std::collections::HashMap;
use std::sync::RwLock;
use std::{thread, time as stdTime};
use tera::Tera;

use tracing::{debug, info, Level};

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
    static ref CACHE: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

const FORMAT_STR: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// BotId feishu webhook group bot id addr
    #[clap(short, long, default_value = "")]
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

    /// Api Version version of feishu api
    #[clap(long = "av", default_value = "v1")]
    api_version: String,

    /// AppSecret secret of feishu app for get user open id
    #[clap(short = 's', long, default_value = "")]
    app_secret: String,

    /// Verbose show verbose log
    #[clap(short, long)]
    verbose: bool,
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_timer(OffsetTime::new(
            offset!(+8),
            format_description::parse(FORMAT_STR).expect("parse format error"),
        ))
        .with_max_level(if args.verbose {
            Level::DEBUG
        } else {
            Level::INFO
        })
        .init();

    let sdk = LarkSdk::new(
        args.app_id,
        args.app_secret,
        args.cache_capacity,
        args.bot_id,
        args.api_version,
    )
    .await;

    tokio::spawn(refresh_token(sdk.clone()));

    let figment = rocket::Config::figment()
        .merge(("address", args.address))
        .merge(("port", args.port));

    let _ = rocket::custom(figment)
        .manage(sdk)
        .mount("/", routes![index, group_message, message])
        .register("/", catchers![not_found])
        .launch()
        .await;
}

async fn refresh_token(sdk: LarkSdk) {
    let mut interval = 0;

    loop {
        let dur = stdTime::Duration::from_secs(interval);
        thread::sleep(dur);

        debug!("refresh_token... ");

        interval = if let Ok(t) = sdk.get_token().await {
            CACHE
                .write()
                .unwrap()
                .insert("token".to_string(), t.tenant_access_token.clone());

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

        info!(
            "current token is {} will refresh after {}s",
            sdk.config.get(&"token".to_string()).unwrap(),
            interval
        );
    }
}
