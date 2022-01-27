use chrono::Local;

use tracing::*;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F %T%.3f"))
    }
}

// 通过instrument属性，直接让整个函数或方法进入span区间，且适用于异步函数async fn fn_name(){}
// 参考：https://docs.rs/tracing/latest/tracing/attr.instrument.html
// #[tracing::instrument(level = "info")]
#[instrument]
fn test_trace(n: i32) {
    // #[instrument]属性表示函数整体在一个span区间内，因此函数内的每一个event信息中都会额外带有函数参数
    // 在函数中，只需发出日志即可
    event!(Level::TRACE, answer = 42, "trace2: test_trace");
    trace!(answer = 42, "trace1: test_trace");
    info!(answer = 42, "info1: test_trace");
}

#[cfg(test)]
#[test]
fn trace_log() {
    use tracing::metadata::LevelFilter;
    use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, Layer};

    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    let file_appender = tracing_appender::rolling::daily("log", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::registry()
        .with(
            fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_timer(LocalTimer)
                .with_filter(LevelFilter::TRACE),
        )
        .with(
            fmt::Layer::new()
                .with_ansi(false)
                .with_timer(LocalTimer)
                .with_writer(non_blocking)
                .with_filter(LevelFilter::TRACE),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    // let format = tracing_subscriber::fmt::format()
    // .with_level(true)
    // .with_target(true)
    // .with_timer(LocalTimer);
    // 初始化并设置日志格式(定制和筛选日志)
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::TRACE)
    //     .with_writer(io::stdout) // 写入标准输出
    //     // .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
    //     // .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
    //     .event_format(format)
    //     .init();

    // test_trace(33);
    trace!("tracing-trace");
    debug!("tracing-debug");
    info!("tracing-info");
    warn!("tracing-warn");
    error!("tracing-error");
}

#[test]
fn offset_datetime() {
    use std::time::SystemTime;
    use time::{format_description, macros::offset, OffsetDateTime};

    println!("{:?}", SystemTime::now());

    let now = time::OffsetDateTime::now_local();
    println!("Hello, world! {:?}", now);

    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+8));

    println!("{:?}", now.format(&format).unwrap());
    println!("{:?}", now.unix_timestamp());
    println!("{:?}", now.unix_timestamp_nanos());
    println!(
        "{:?}",
        OffsetDateTime::from_unix_timestamp(now.unix_timestamp()).unwrap()
    );
}

#[test]
fn test_aes_cbc() {
    use crate::util::aes_cbc;

    let plaintext = "予定表～①??????だ";
    let key = "vaq9ohuOdiYf8Q9UlxSz6bF5ZQqjPmpO";
    let enc = aes_cbc::encrypt(key, plaintext);

    let dec = aes_cbc::decrypt(key, &enc);
    assert_eq!(plaintext, dec);
}

#[test]
fn test_aes2() {
    use aes::Aes256;
    use block_modes::block_padding::NoPadding;
    use block_modes::{BlockMode, Cbc};
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    type Aes256Cbc = Cbc<Aes256, NoPadding>;

    hasher.update(b"vaq9ohuOdiYf8Q9UlxSz6bF5ZQqjPmpO");
    let key = hasher.finalize();

    // let key = "vaq9ohuOdiYf8Q9UlxSz6bF5ZQqjPmpO".as_bytes();
    println!("key: {:?}", key);
    // let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    // let iv = [0u8; 16];

    // let plaintext = b"Hello world!";
    // let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // // buffer must have enough space for message+padding
    // let mut buffer = [0u8; 32];
    // // copy message to the buffer
    // let pos = plaintext.len();
    // buffer[..pos].copy_from_slice(plaintext);
    // let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    // // assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));
    // let bs64text = base64::encode(ciphertext);
    // // let bs64text = "jN3VjmCYyHaxl5bUNVwLMA==";
    let bs64text = "33sim+MR8JEaG+S9PqirSwlM/MrVbytGQlm/VhKBUqphqU0foD5fnpFa+vphyi9T4l3uAi6KrDaNQkGMHSQYLhrm3xFwHOqb1VnxP6q4QeJtHv9dO/lli6Re3OtbXd8t0NXPqitj9EmOl7kuaCAai/t77+r4V/yuQIKRidB1eYRnHdGWC4cJl61F9NNpb9iY";
    let dbs64 = base64::decode(bs64text).unwrap();
    let iv = &dbs64[..16];

    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let ciphertext = &dbs64[16..];
    let mut buf = ciphertext.to_vec();

    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

    // assert_eq!(decrypted_ciphertext, plaintext);
    println!(
        "decrypted_ciphertext: {}",
        String::from_utf8(decrypted_ciphertext.to_vec()).unwrap()
    )
}
