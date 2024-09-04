use actix_web::{App, HttpServer};
use server::*;
use session::session_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logging)
            .wrap(session_middleware())
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
