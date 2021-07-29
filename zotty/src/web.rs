use tracing::debug;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub async fn rocket() {
    debug!("Firing up webserver");
    println!("TEST");
    rocket::build()
        .mount("/", routes![index])
        .launch().await;
}
