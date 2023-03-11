
use chrono::Utc;
use chrono::Local;
use chrono::Duration;

fn main() {
    let utc_time = Utc::now();
    let local_time = Local::now();

    println!("UTC time is: {}", utc_time);
    println!("Local time is: {}", local_time);

    Duration::minutes(1);
}
