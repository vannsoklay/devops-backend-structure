use actix_web::{App, HttpServer};
use actix_cors::Cors;
use server::*;
use actix_web::http::header;
use session::session_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start Actix Web server
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST", "DELETE", "UPDATE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ACCEPT,
            ])
            .allow_any_header()
            .max_age(3600)
            .supports_credentials();
        App::new()
            .wrap(cors)
            .wrap(Logging)
            .wrap(session_middleware())
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
