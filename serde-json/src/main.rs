use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    ip: String,
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
enum Location {
    US = 1,
    Canada = 2,
    Europe = 3
}

fn main() {
    let json_str = r#"{
            "ip": "138.23.67",
            "location": "US"
        }"#;

    let sample2 = r#"{
            "ip": "138.23.67",
            "location": "Canada"
        }"#;

    let result: Result<ServerInfo, serde_json::Error> = serde_json::from_str(sample2);

    match result {
        Ok(server_info) => {
            println!("serialized json successfully to server info: {server_info:#?}")
        }
        Err(e) => println!("deserializing from json with error: {e}"),
    }
}
