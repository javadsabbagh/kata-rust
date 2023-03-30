

fn main() {

    // default log level for env_logger is error, unless it is set in RUST_LOG environment variable.
    // e.g. RUST_LOG=info cargo run application
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // To see log messages lower than error use `RUST_LOG=info cargo run`
    log::info!("Just an info message...");
    log::warn!("Caution: take this seriously...");
    log::error!("Error occurred...");
}
