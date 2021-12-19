pub mod search_api;
// pub mod search_api_and_register;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").route("", web::get().to(search_api::route)));
}
