use std::env;

use time::{format_description, macros::offset, OffsetDateTime};
use tracing::info;
use tracing_subscriber::fmt::time::OffsetTime;

const FORMAT_STR: &str = "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]";

#[cfg(test)]
#[test]
fn offset_datetime() {
    let timer = OffsetTime::new(offset!(+8), format_description::parse(FORMAT_STR).unwrap());
    env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt().with_timer(timer).init();

    let format = format_description::parse(FORMAT_STR).unwrap();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+8));
    info!("{:?}", now.format(&format).unwrap());

    info!("{:?}", now.unix_timestamp());
    info!("{:?}", now.unix_timestamp_nanos());
    info!(
        "{:?}",
        OffsetDateTime::from_unix_timestamp(now.unix_timestamp()).unwrap()
    );
}
