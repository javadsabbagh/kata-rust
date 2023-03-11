
use chrono::Utc;
use chrono::Local;
use chrono::Duration;

fn main() {
    let utc_time = Utc::now();
    let local_time = Local::now();

    println!("UTC time is: {}", utc_time);
    println!("Local time is: {}", local_time);

    // we can use +/- operator in chrono objects
    let future_time = local_time + Duration::minutes(10);
    println!("Future time is: {}", future_time);

    // formatting
    println!("Current time is: {}", local_time.format("%y-%m-%d"));
}
