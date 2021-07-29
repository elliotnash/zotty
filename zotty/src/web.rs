use rocket::{Config, Method, Request, Response, fairing::{Fairing, Info, Kind}, http::Header, serde::json::Json};
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions};
use serde::Serialize;
use crate::CONFIG;

#[get("/ping")]
async fn ping() -> Json<OAuthInfo> {
    Json(OAuthInfo{
        api_url: &CONFIG.get().unwrap().web.oauth.api_url,
        client_id: &CONFIG.get().unwrap().web.oauth.client_id
    })
}

#[derive(Serialize)]
struct OAuthInfo{
    pub api_url: &'static str,
    pub client_id: &'static str
}

pub async fn rocket() {
    let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    // load rocket options from config
    let mut config = Config::default();
    config.port = CONFIG.get().unwrap().web.port;
    rocket::build()
        .configure(config)
        .mount("/api", routes![ping])
        .launch().await.expect("Failed to create rocket server");
}
