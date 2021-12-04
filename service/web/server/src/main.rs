// external crate
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use twitter_search::database_utils::pool::env_database_url;
use twitter_search::routes;
use twitter_search::twitter::{hit_api_and_register_tweet, run_search};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_url = env_database_url();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    let bind = format!("0.0.0.0:{}", port);

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| -> bool {
                dotenv().ok();
                env::var("allowed_origin")
                    .expect("allowed_origin must be set")
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .any(|env_origin| env_origin == origin)
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/tweets").configure(routes::tweets::config))
            .service(web::scope("/twitter_api").configure(routes::hit_twitter_api::config))
            .service(web::resource("/twitter_search").route(web::get().to(run_search)))
            .service(
                web::resource("/register_tweet").route(web::get().to(hit_api_and_register_tweet)),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
