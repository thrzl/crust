use actix_web::{get, App, web, HttpServer, Responder, HttpResponse, http, middleware::Logger};
use reqwest::get;
use std::collections::HashMap;
use miniserde::{Serialize, Deserialize, json};
use cached::proc_macro::cached;
use env_logger;
use select::document::Document;
use select::predicate::{Class, Name};


#[cached(size=1000)]
async fn get_pinned(u: String) -> Vec<HashMap<String, String>> {
    let resp = get(format!("https://github.com/{u}")).await.unwrap();
    let document = Document::from(&resp.text().await.unwrap().to_owned()[..]);
    let mut repos: Vec<_> = Vec::new();
    document.find(Class("pinned-item-list-item"))
        .for_each(|node| {
            let mut repo = HashMap::new();
            node.find(Class("repo"))
                .for_each(|node| {
                    repo.insert("name".to_string(), node.text().to_string());
                });
            let repoData = get("https://api.github.com/repos/".to_string() + &repo["name"]).await.unwrap();
            repos.push(repo);
        });
    
    repos
}

#[get("/")]
async fn index() -> Result<HttpResponse, http::Error> {
    Ok(HttpResponse::PermanentRedirect()
    .append_header(("Location", "https://crust.terabyteis.me")).finish())
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
        uuid,
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
        }
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await        
}
