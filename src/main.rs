#![feature(decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate rocket_contrib;
extern crate memoize as m;
extern crate serde;
extern crate lru;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use serde::Serialize;
use lru::LruCache;
use std::cell::UnsafeCell;


// let http: reqwest::Client = reqwest::Client::new();

#[derive(Serialize)]
struct User {
  name: String,
  uuid: String,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/nametest")]
// fn nametest(name: String) -> String {
//     println!("Hello {name}")
// }

#[get("/user/<name>")]
fn user_f_name(cache: rocket::State<&mut LruCache<String, User>>, http: rocket::State<reqwest::blocking::Client>, name: String) -> Json<User> {
    
    let req_url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let u: User;
    let resp_data: HashMap<String, String>;
    let mut c = &*cache;
    if (*c).get_mut(&name).is_none() {
        let req_url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
        let resp = http.get(req_url).send();
        let resp_data = resp.unwrap().json::<HashMap<String, String>>().unwrap();
        let uuid = resp_data.get("id").unwrap();
        let u = User {
            name: name,
            uuid: uuid.to_string(),
        };
        cache.put(name, u);
    } else {
        ()
    };
    format!("Hello {name}, your uuid is {uuid}", name=name, uuid=u.uuid);
    let user = User{name: resp_data.get("name").unwrap().to_string(), uuid: resp_data.get("id").unwrap().to_string()};
    Json(user)
}

fn main() {
    let http: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let mut user_cache: LruCache<String, User> = LruCache::new(100);
    rocket::ignite().manage(http).manage(user_cache).mount("/", routes![index, user_f_name]).launch();
    println!("🪨 ready")
}