use actix_web::{get, App, web, HttpServer, Responder, HttpResponse, http, middleware::{Logger, ErrorHandlers}};
use reqwest::get;
use std::{collections::HashMap, sync::mpsc::RecvError};
use miniserde::{Serialize, Deserialize, json};
use cached::proc_macro::cached;
use env_logger;
use uuid::Uuid;
use async_minecraft_ping::ConnectionConfig;
use fastping_rs::{Pinger, PingResult};

#[derive(Serialize, Deserialize)]
struct User {
  name: String,
  uuid: String,
}

#[derive(Serialize, Deserialize)]
struct Server {
    ping: u64,
    host: String,
    ipaddr: String,
    players: u32,
    maxplayers: u32,
    favicon: String,
}

#[cached(size=1000)]
async fn request(url: String) -> HashMap<String, String> {
    let resp = get(&url).await.unwrap();
    let resp_data = json::from_str(&resp.text().await.unwrap()).unwrap();
    resp_data
}

async fn we_died() -> impl Responder {
    let mut resp = HashMap::new();
    resp.insert("error", "We died");
    resp.insert("code", "500");
    HttpResponse::InternalServerError().content_type("application/json").body(json::to_string(&resp))
}

#[get("/")]
async fn index() -> Result<HttpResponse, http::Error> {
    Ok(HttpResponse::PermanentRedirect()
    .append_header(("Location", "https://crust.terabyteis.me")).finish())
}

#[get("/server/{hostname}")]
async fn server_info(hostname: web::Path<String>) -> impl Responder {
    // use async-minecraft-ping to ping server
    let mut config = ConnectionConfig::build(hostname.to_string());
    let connection = config.connect().await.unwrap();
    let status = connection.status().await.unwrap();
    let (pinger, results) = Pinger::new(None, Some(56)).unwrap();
    pinger.add_ipaddr(&hostname.to_string());
    pinger.run_pinger();
    // let motd = match status.status.description {
    //     Object { text } => {
    //         text
    //     } else {
    //         None
    //     }
    // };
    let res = match results.recv() {
        Ok(result) => match result {
            PingResult::Idle { addr: _ } => {
                let mut resp = HashMap::new();
                resp.insert("error", "Requested server is offline");
                resp.insert("code", "503");
                HttpResponse::ServiceUnavailable().content_type("application/json").body(json::to_string(&resp))
            },
            PingResult::Receive { addr, rtt } => {
                let server = Server {
                    ping: rtt.as_secs()*1000,
                    host: hostname.to_string(),
                    ipaddr: addr.to_string(),
                    players: status.status.players.online,
                    maxplayers: status.status.players.max,
                    favicon: status.status.favicon.unwrap(),
                };
                HttpResponse::Ok().content_type("application/json").body(json::to_string(&server))
            }
        },
        _ => {
            let mut resp = HashMap::new();
            resp.insert("error", "Something went wrong");
            resp.insert("code", "500");
            HttpResponse::InternalServerError().content_type("application/json").body(json::to_string(&resp))
        },
    };
    res
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(r#"<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
}

#[get("/user/{name}")]
async fn user(name: web::Path<String>) -> impl Responder {
    let uuid = Uuid::parse_str(&name);
    let resp_data = if uuid.is_ok() {
        let r: HashMap<String, String> = request(format!("https://api.mojang.com/user/profile/{}", &uuid.unwrap())).await;
        r
    } else {
        let r: HashMap<String, String> = request(format!("https://api.mojang.com/users/profiles/minecraft/{}", name)).await;
        r
    };
    let uuidm = Uuid::parse_str(resp_data.get("id").unwrap());
    let uuid = if uuidm.is_ok() {
        let u = uuidm.unwrap();
        u.to_hyphenated().to_string()
    } else {
        let u = resp_data.get("id").unwrap();
        u.to_owned()
    };
    let u = User {
        name: resp_data.get("name").unwrap().to_owned(),
        uuid: uuid,
    };
    json::to_string(&u)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Starting on port {}", 8080);
    HttpServer::new(|| {App::new()
            .wrap(Logger::default())
            // .service(index)
            .service(hello)
            .service(user)
            .service(server_info)
        }
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await        
}
