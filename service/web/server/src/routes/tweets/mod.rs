pub mod delete;
pub mod get;
pub mod post_record;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::route))
            .route("", web::post().to(post_record::route))
            .route("", web::delete().to(delete::route)),
    );
}
