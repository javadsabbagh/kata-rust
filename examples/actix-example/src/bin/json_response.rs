#![allow(unused_imports)]

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize)]
struct ServerList {
    servers: Vec<ServerInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    id: String,
    /* UUID of the server, TODO use UUID*/
    ip: String,
    /* Note:Rust fields are snake_case by convention, however json convention in naming fields is camelCase! */
    resource_info: ResourceInfo,
}

#[derive(Serialize)]
struct ResourceInfo {
    /* number of cpu cores */
    cpu: u8,
    /* memory size in megabytes */
    memory: u16,
    /* disk size in megabytes */
    disk: u64,
}

#[get("/get-all")]
async fn get_all() -> impl Responder {
    let list = ServerList {
        servers: vec![ServerInfo {
            id: String::from("a7957610-bc59-11ed-8f6c-88a4c2e3226d"),
            ip: String::from("185.121.123.90"),
            resource_info: ResourceInfo {
                cpu: 4_u8,
                memory: 3_000_u16,
                disk: 100_000_u64,
            },
        }],
    };

    web::Json(list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().app_data({}).service(get_all))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
