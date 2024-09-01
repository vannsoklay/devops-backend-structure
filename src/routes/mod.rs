use actix_web::web;

pub mod user_routes;
pub mod item_routes;
pub mod auth_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    auth_routes::configure(cfg);
    user_routes::configure(cfg);
    item_routes::configure(cfg);
}