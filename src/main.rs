use actix_cors::Cors;
use actix_web::{
    dev::Service as _, get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use database::mongodb::{DbName, MongoClient, MongoClientBuilder, Url};
use futures_util::future::FutureExt;

use env_logger::Env;
use handlers::error_handler::Errors;

pub mod config;
pub mod database;
pub mod handlers;
pub mod helpers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod traits;

use crate::config::{DATABASE_NAME, MONGO_URI};

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

async fn build_mongo_client() -> Result<MongoClient, Errors> {
    let url = Url::new(MONGO_URI.to_string());
    let db_name = DbName::new(DATABASE_NAME.to_string());

    MongoClientBuilder::<Url, DbName>::url(url, db_name)
        .await?
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::load_env();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let mongo_client = build_mongo_client()
        .await
        .expect("Database connection error!");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(mongo_client.clone()))
            .service(health_check)
            .service(routes::routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
