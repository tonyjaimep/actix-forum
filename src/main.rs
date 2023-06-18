use actix_web::{
    http::StatusCode,
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};
use diesel::r2d2;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;

use actix_forum::db_pool::DbPool;
use actix_forum::modules::forum;
use actix_forum::modules::post;
use actix_forum::modules::thread;

async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND).body("Not found")
}

fn get_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let connection_manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Could not build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let connection_pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(forum::service::setup())
            .service(thread::service::setup())
            .service(post::service::setup())
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
