use rocket::Config;

#[get("/ping")]
async fn ping() -> &'static str {
    "pong"
}

pub async fn rocket() {
    let config = Config::default();
    rocket::build()
        .configure(config)
        .mount("/api", routes![ping])
        .launch().await.expect("Failed to create rocket server");
}
