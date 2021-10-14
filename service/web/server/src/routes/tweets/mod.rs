pub mod post_record;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").route("", web::post().to(post_record::route)));
}
