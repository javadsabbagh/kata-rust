
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono::Local;
use chrono::Duration;

use std::time::Instant;

fn distract_instants(in1: Instant, in2: Instant) {
    let d = (in1 - in2);
    println!("{:?}", d);
}

fn main() {
    let utc_time = Utc::now();
    let local_time = Local::now();

    println!("UTC time is: {}", utc_time);
    println!("Local time is: {}", local_time);

    // we can use +/- operator in chrono objects
    let future_time = local_time + Duration::minutes(10);
    println!("Future time is: {}", future_time);

    // formatting
    println!("Current time is: {}", local_time.format("%y-%m-%d %H-%M-%S"));

    // parsing
    // 1- directly from string: it returns Result<DateTime, Err>
    println!("Unix epoch time is: {:?}", "1970-01-01T00:00:00Z".parse::<DateTime<Utc>>());
    println!("Unix epoch time in local time is: {:?}", "1970-01-01T00:00:00Z".parse::<DateTime<Local>>());
    //println!("Unix epoch time in local time is: {:?}", "1970-01-01T00:00:00Z".parse::<DateTime<Local>>());  // Err(ParseError(NotEnough))


    // 2- By calling associate function or method
    dbg!(DateTime::parse_from_str("1970/01/01", "%y/%m/%d"));

    //dbg!(Utc.from_utc_datetime("1970-01-01"));

}
