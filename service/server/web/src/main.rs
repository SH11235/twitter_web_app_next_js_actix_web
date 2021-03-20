use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger;
use std::env;
use twitter_search::twitter::{run_search, write_tweet};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/twitter_search").route(web::get().to(run_search)))
            .service(web::resource("/write_tweet").route(web::get().to(write_tweet)))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .await
}
