use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use crate::CONFIG;

#[get("/api/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(
        OAuthInfo{
            api_url: &CONFIG.get().unwrap().web.oauth.api_url,
            client_id: &CONFIG.get().unwrap().web.oauth.client_id
        }
    )
}

#[derive(Serialize)]
struct OAuthInfo{
    pub api_url: &'static str,
    pub client_id: &'static str
}

pub async fn run() {
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
        .workers(8)
        .bind(("127.0.0.1", CONFIG.get().unwrap().web.port))
        .unwrap().run().await.unwrap();
}
