use clap::Parser;
use feishu::sdk;
use rocket::{
    catch, catchers, get, post, routes,
    serde::json::{serde_json::json, Value},
};
use server::{index, not_found};

mod feishu;
mod server;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Webhook feishu webhook addr
    #[clap(
        short,
        long,
        default_value = "https://open.feishu.cn/open-apis/bot/v2/hook/d66fffcc-c6af-406d-a4c3-96cb112f9fca"
    )]
    webhook: String,

    /// IP address to serve on
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Port to serve on
    #[clap(short, long, default_value_t = 8000)]
    port: u16,

    /// AppID id of feishu app for get user open id
    #[clap(short = 'i', long)]
    app_id: String,

    /// AppSecret secret of feishu app for get user open id
    #[clap(short = 's', long)]
    app_secret: String,

    /// Verbose show verbose log"
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    match launch_rocket() {
        Ok(_) => print!("server start "),
        Err(err) => panic!("{}", err),
    }
}

#[rocket::main]
pub async fn launch_rocket() -> Result<(), rocket::Error> {
    let args = Args::parse();

    let figment = rocket::Config::figment()
        .merge(("address", args.address))
        .merge(("port", args.port));

    // let sdk = sdk::Sdk::new(args.app_id, args.app_secret);
    // println!("{}", sdk.token);

    rocket::custom(figment)
        // .mount("/", routes![index])
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .launch()
        .await
}
