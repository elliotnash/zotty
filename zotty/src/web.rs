use rocket::{
    Config,
    serde::json::Json
};
use serde::Serialize;
use crate::CONFIG;

#[get("/ping")]
async fn ping() -> Json<OAuthInfo> {
    Json(OAuthInfo{
        api_url: CONFIG.get().unwrap().web.oauth.api_url.clone(),
        client_id: CONFIG.get().unwrap().web.oauth.client_id.clone()
    })
}

#[derive(Serialize)]
struct OAuthInfo{
    pub api_url: String,
    pub client_id: String
}

pub async fn rocket() {
    let mut config = Config::default();
    config.port = CONFIG.get().unwrap().web.port;
    rocket::build()
        .configure(config)
        .mount("/api", routes![ping])
        .launch().await.expect("Failed to create rocket server");
}
