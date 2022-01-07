#[cfg(test)]
#[test]
fn offset_datetime() {
    use time::{format_description, macros::offset, OffsetDateTime};

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
