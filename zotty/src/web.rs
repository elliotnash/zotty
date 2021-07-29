use rocket::Config;
use crate::CONFIG;

#[get("/ping")]
async fn ping() -> &'static str {
    "pong"
}

pub async fn rocket() {
    let mut config = Config::default();
    config.port = CONFIG.get().unwrap().web.port;
    rocket::build()
        .configure(config)
        .mount("/api", routes![ping])
        .launch().await.expect("Failed to create rocket server");
}
