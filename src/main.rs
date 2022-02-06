use actix_web::{get, App, web, HttpServer, Responder, HttpResponse, http, middleware::Logger};
use reqwest::get;
use std::collections::HashMap;
use miniserde::{Serialize, Deserialize, json};
use cached::proc_macro::cached;
use env_logger;

#[derive(Serialize, Deserialize)]
struct User {
  name: String,
  uuid: String,
}

#[cached(size=1000)]
async fn request(url: String) -> HashMap<String, String> {
    // if cache.find(url).is_some() {
    //     println!("Cache hit for {}", url);
    //     return cache.get(&url).unwrap().clone();
    // }
    let resp = get(&url).await.unwrap();
    // println!("Cache miss for {}", url);
    // let resp_data = resp.text::<HashMap<String, String>>().await.unwrap();
    let resp_data = json::from_str(&resp.text().await.unwrap()).unwrap();
    // cache.insert(url, resp_data.clone());
    resp_data
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(r#"<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
}

#[get("/user/{name}")]
async fn user(name: web::Path<String>) -> impl Responder {
    // lazy_static! {
    //     static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(Client::new()).with(
    //         Cache(
    //             HttpCache {
    //                 mode: CacheMode::NoCache,
    //                 manager: CACacheManager::default(),
    //                 options: None,
    //             }
    //         )
    //     ).build();
    // }
    let resp_data = request(format!("https://api.mojang.com/users/profiles/minecraft/{}", name)).await;
    let uuid = resp_data.get("id").unwrap();
    let u = User {
        name: name.to_string(),
        uuid: uuid.to_string(),
    };
    json::to_string(&u)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Starting on port {}", 8080);
    HttpServer::new(
        || {App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(user)
        }
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
        
}
