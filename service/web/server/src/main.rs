// external crate
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger;
use std::env;
use twitter_search::twitter::{hit_api_and_register_tweet, register_favorite_tweet, run_search};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/twitter_search").route(web::get().to(run_search)))
            .service(
                web::resource("/register_tweet").route(web::get().to(hit_api_and_register_tweet)),
            )
            .service(register_favorite_tweet)
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .await
}
