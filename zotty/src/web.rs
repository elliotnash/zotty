use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, post, web};
use actix_cors::Cors;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::CONFIG;

lazy_static! {
    static ref REQWEST: Client = Client::new();
}

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
    if resp.status().is_success() {
        let resp_json: AccessTokenResponse = resp.json().await.unwrap();
        Ok(HttpResponse::Ok().json(resp_json))
    } else {
        let resp_json: DiscordError = resp.json().await.unwrap();
        Ok(HttpResponse::BadRequest().json(resp_json))
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
#[serde(untagged)]
enum DiscordError{
    OAuthError(OAuthError),
    GatewayError(GatewayError)
}
#[derive(Serialize, Deserialize, Debug)]
struct OAuthError{
    pub error: String,
    pub error_description: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GatewayError{
    pub code: i32,
    pub message: String,
}

#[get("/api/users/@me")]
async fn user_me(request: HttpRequest) -> Result<impl Responder> {
    let auth_header = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let resp = REQWEST.get(format!("{}/users/@me", CONFIG.get().unwrap().web.oauth.api_url))
        .header("Authorization", auth_header)
        .send().await.unwrap();
    if resp.status().is_success() {
        let resp_json: DiscordUser = resp.json().await.unwrap();
        Ok(HttpResponse::Ok().json(resp_json))
    } else {
        let resp_json: DiscordError = resp.json().await.unwrap();
        Ok(HttpResponse::Forbidden().json(resp_json))
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String
}

pub async fn run() {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ping)
            .service(login)
            .service(user_me)
    })
        .workers(8)
        .bind(("127.0.0.1", CONFIG.get().unwrap().web.port))
        .unwrap().run().await.unwrap();
}
