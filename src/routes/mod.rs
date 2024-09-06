use actix_web::web;

pub mod auth_route;
pub mod user_route;
pub mod post_route;
pub mod item_route;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    auth_route::configure(cfg);
    user_route::configure(cfg);
    post_route::configure(cfg);
    item_route::configure(cfg);
}