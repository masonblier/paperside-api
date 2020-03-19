#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod app; 
mod schema;

use app::config;
use app::controllers::*;

/// main
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    println!("db: {}", config::DATABASE_URI.as_str());
    let manager = ConnectionManager::<PgConnection>::new(config::DATABASE_URI.as_str());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server
    println!("Starting server at: {}", config::BIND_ADDRESS.as_str());
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // identity
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(config::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(config::APP_DOMAIN.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            // json request parsing config
            .data(web::JsonConfig::default().limit(4096))

            // routes
            .service(
                web::resource("/register")
                    .route(web::post().to(registrations_controller::register_user)),
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(auth_controller::login))
                    .route(web::delete().to(auth_controller::logout))
                    .route(web::get().to(auth_controller::get_me)),
            )
    })
    .bind(config::BIND_ADDRESS.as_str())?
    .run()
    .await
}
