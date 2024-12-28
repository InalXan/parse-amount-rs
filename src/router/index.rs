use actix_web::web;
#[path = "../../src/utils/app.rs"]
mod app;
use app::index;

pub fn configure_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}
