use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, post, web};
use actix_cors::Cors;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;
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

#[post("/api/login")]
async fn login(cred: web::Json<LoginCredentials>) -> Result<impl Responder> {
    lazy_static! {
        static ref REQWEST: Client = Client::new();
    }
    let token_request = AccessTokenRequest {
        client_id: &CONFIG.get().unwrap().web.oauth.client_id,
        client_secret: &CONFIG.get().unwrap().web.oauth.client_secret,
        grant_type: "authorization_code",
        code: cred.code.clone(),
        redirect_uri: cred.redirect_uri.clone()
    };
    let resp = REQWEST.post(format!("{}/oauth2/token", CONFIG.get().unwrap().web.oauth.api_url))
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(&token_request).unwrap())
        .send().await.unwrap();
    let resp_text = resp.text().await.unwrap();
    let token_response_result: Result<AccessTokenResponse, serde_json::Error> = serde_json::from_str(&resp_text);
    if let Ok(token_response) = token_response_result {
        Ok(HttpResponse::Ok().json(token_response))
    } else {
        let error: DiscordError = serde_json::from_str(&resp_text).unwrap();
        Ok(HttpResponse::Ok().json(error))
    }
}
#[derive(Deserialize, Debug)]
struct LoginCredentials{
    pub code: String,
    pub redirect_uri: String
}
#[derive(Serialize, Deserialize, Debug)]
struct AccessTokenRequest{
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub grant_type: &'static str,
    pub code: String,
    pub redirect_uri: String
}
#[derive(Serialize, Deserialize, Debug)]
struct AccessTokenResponse{
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String
}
#[derive(Serialize, Deserialize, Debug)]
struct DiscordError{
    pub error: String,
    pub error_description: String,
}

pub async fn run() {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ping)
            .service(login)
    })
        .workers(8)
        .bind(("127.0.0.1", CONFIG.get().unwrap().web.port))
        .unwrap().run().await.unwrap();
}
