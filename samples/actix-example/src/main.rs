#![allow(unused_imports)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct ServerList {
    serverInfos: Vec<ServerInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    id: String,
    /* UUID of the server, TODO use UUID*/
    ip: String,
    resourceInfo: ResourceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResourceInfo {
    cpu: u8,    /* number of cpu cores */
    memory: u16, /* memory size in megabytes */
    disk: u64  /* disk size in megabytes */
}

#[get("/get-all")]
async fn get_all() -> ServerList {
    ServerList {
        serverInfos: vec![ServerInfo {
            id: String::from("a7957610-bc59-11ed-8f6c-88a4c2e3226d"),
            ip: String::from("185.121.123.90"),
            resourceInfo: ResourceInfo {
                cpu: 4_u8,
                memory: 3_000_u16,
                disk: 100_000_u64,
            },
        }],
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().app_data({}).service(get_all))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}