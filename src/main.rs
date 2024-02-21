use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use database::mongodb::{DbName, MongoClient, MongoClientBuilder, Url};
use dotenv::dotenv;
use env_logger::Env;
use handlers::error_handler::Errors;
use std::env;

pub mod database;
pub mod handlers;
pub mod traits;

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

async fn build_mongo_client() -> Result<MongoClient, Errors> {
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URI").unwrap_or_default();
    let database_name = env::var("DATABASE_NAME").unwrap_or_default();

    let url = Url::new(mongo_uri);
    let db_name = DbName::new(database_name);

    MongoClientBuilder::<Url, DbName>::url(url, db_name)
        .await?
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
