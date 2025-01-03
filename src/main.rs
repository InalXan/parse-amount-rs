// input: "430 euro"
// "430 euro" -> 430, "euro"

use actix_web::{App, HttpServer};
// router
#[path = "../src/router/index.rs"]
mod index;
use index::configure_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // host configure
    let host = "127.0.0.1:8081";
    let host_err = "server connection error";

    HttpServer::new(move || App::new().configure(configure_router))
        .bind(host)
        .expect(host_err)
        .workers(2)
        .run()
        .await
}
