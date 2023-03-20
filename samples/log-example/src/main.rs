
fn main() {
    env_logger::init();

    // To see log messages lower than error use `RUST_LOG=info cargo run`
    log::info!("Just an info message...");
    log::warn!("Caution: take this seriously...");
    log::error!("Error occurred...");
}
