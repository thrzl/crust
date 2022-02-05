use actix_web::{get, App, web, HttpServer, Responder, HttpResponse, http};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache};
use std::collections::HashMap;
use serde::Serialize;
use lazy_static::lazy_static;


#[derive(Serialize)]
struct User {
  name: String,
  uuid: String,
}

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(r#"<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)
}

#[get("/user/{name}")]
async fn user(name: web::Path<String>) -> impl Responder {
    lazy_static! {
        static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(Client::new()).with(
            Cache(
                HttpCache {
                    mode: CacheMode::NoCache,
                    manager: CACacheManager::default(),
                    options: None,
                }
            )
        ).build();
    }
    let req_url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let resp = CLIENT.get(req_url).send().await.unwrap();
    let resp_data = resp.json::<HashMap<String, String>>().await.unwrap();
    let uuid = resp_data.get("id").unwrap();
    let u = User {
        name: name.to_string(),
        uuid: uuid.to_string(),
    };
    web::Json(u)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static! {
        static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(Client::new()).with(
            Cache(
                HttpCache {
                    mode: CacheMode::NoCache,
                    manager: CACacheManager::default(),
                    options: None,
                }
            )
        ).build();
    };
    HttpServer::new(|| App::new().service(index).service(user))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
