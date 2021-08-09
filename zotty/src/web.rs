use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, post, web};
use actix_cors::Cors;
use lazy_static::lazy_static;
use reqwest::Client;
use array_tool::vec::Intersect;
use actix_files::{Files, NamedFile};
use serde::{Deserialize, Serialize};
use crate::{CACHE, CONFIG, HOME_DIR};

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
    pub error_description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct GatewayError{
    pub code: i32,
    pub message: String,
}

#[post("/api/refresh")]
async fn refresh(cred: web::Json<RefreshCredentials>) -> Result<impl Responder> {
    let token_request = RefreshTokenRequest {
        client_id: &CONFIG.get().unwrap().web.oauth.client_id,
        client_secret: &CONFIG.get().unwrap().web.oauth.client_secret,
        grant_type: "refresh_token",
        refresh_token: cred.refresh_token.clone()
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
struct RefreshCredentials{
    pub refresh_token: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct RefreshTokenRequest{
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub grant_type: &'static str,
    pub refresh_token: String
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
        Ok(HttpResponse::Unauthorized().json(resp_json))
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String
}

#[get("/api/users/@me/guilds")]
async fn guild_me(request: HttpRequest) -> Result<impl Responder> {
    // fetch users guild from discord
    let auth_header = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let resp = REQWEST.get(format!("{}/users/@me/guilds", CONFIG.get().unwrap().web.oauth.api_url))
        .header("Authorization", auth_header)
        .send().await.unwrap();
    if resp.status().is_success() {
        let user_pguilds = resp.json::<Vec<PartialGuild>>().await.unwrap();
        let user_guilds = user_pguilds.iter().map(|p| p.id.clone()).collect();
        // fetch bots guilds from serentiy
        if let Some(cache) = CACHE.get() {
            let bot_guilds: Vec<String> = cache.guilds().await.iter().map(|g| g.to_string()).collect();
            let intersect_guilds = bot_guilds.intersect(user_guilds);
            let intersect_pguilds: Vec<PartialGuild> = user_pguilds.iter().cloned().filter(|g| intersect_guilds.contains(&g.id)).collect();
            Ok(HttpResponse::Ok().json(intersect_pguilds))
        } else {
            Ok(HttpResponse::InternalServerError().body("Server starting"))
        }
        
    } else {
        let resp_json: DiscordError = resp.json().await.unwrap();
        Ok(HttpResponse::Unauthorized().json(resp_json))
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PartialGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: String,
}

pub async fn run() {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        let mut build_dir = HOME_DIR.get().unwrap().clone();
        build_dir.pop();
        build_dir.push("zotty-web");
        build_dir.push("build");
        let mut index_path = build_dir.clone();
        index_path.push("index.html");
        App::new()
            .wrap(cors)
            .service(ping)
            .service(login)
            .service(user_me)
            .service(guild_me)
            .service(refresh)
            .service(Files::new("/", build_dir)
                .index_file("index.html")
                .default_handler(NamedFile::open(index_path).unwrap()))
    })
        .workers(8)
        .bind(("0.0.0.0", CONFIG.get().unwrap().web.port))
        .unwrap().run().await.unwrap();
}
