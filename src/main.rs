use clap::Parser;

use rocket::{catchers, routes};
use server::{index, not_found, send_text};

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

    let figment = rocket::Config::figment()
        .merge(("address", args.address))
        .merge(("port", args.port));

    // let sdk = sdk::Sdk::new(args.app_id, args.app_secret);
    // println!("{}", sdk.token);

    match rocket::custom(figment)
        .mount("/", routes![index, send_text])
        .register("/", catchers![not_found])
        .launch()
        .await
    {
        Ok(_) => {
            todo!()
        }
        Err(err) => panic!("{}", err),
    }
}
